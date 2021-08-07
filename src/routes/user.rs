use crate::auth::{
    gen_token_cookie, AdminAuth, StaffAuth, UserDigest, USER_AUTH_ARGON2_CONFIG, USER_AUTH_SALT,
};
use crate::auxiliary::{GenericError, GenericResult, SuccessResponse};
use crate::auxiliary::{WECHAT_APPID, WECHAT_APPSECRET};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use isahc::{self, AsyncReadResponseExt};

use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use lazy_static;

lazy_static! {
    static ref WECHAT_USERINFO_URL: &'static str =
        "https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}&lang=zh_CN";
}

#[get("/verify_login")]
pub async fn verify_login(
    user_digest: UserDigest,
    db: MainDatabaseConnection,
) -> GenericResult<UserLoggedInDigest> {
    let query_result: User = db
        .run(move |c| {
            database::users::table
                .find(user_digest.user_id)
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(UserLoggedInDigest {
        username: query_result.username,
        user_role: query_result.user_role,
    })
}

#[post("/login", data = "<login_data>")]
pub async fn login(
    db: MainDatabaseConnection,
    login_data: Json<ClientUsernamePasswordData>,
    cookies: &CookieJar<'_>,
) -> GenericResult<UserLoggedInDigest> {
    let username = login_data.username.to_owned();
    match db
        .run(move |c| {
            database::users::table
                .filter(database::users::username.eq_all(username))
                .get_result::<User>(c)
        })
        .await
    {
        Ok(query_result) => match query_result.password_hashed {
            Some(password_hashed) => {
                match argon2::verify_encoded(&password_hashed, &login_data.password.as_bytes()) {
                    Ok(verify_result) => match verify_result {
                        true => match gen_token_cookie(query_result.id, query_result.user_role) {
                            Ok(token_cookie) => {
                                cookies.add(token_cookie);
                                SuccessResponse::build(UserLoggedInDigest {
                                    user_role: query_result.user_role,
                                    username: query_result.username,
                                })
                            }
                            Err(_) => Err(GenericError::ServerInternalError),
                        },
                        false => Err(GenericError::PasswordIncorrectError),
                    },
                    Err(_) => Err(GenericError::AuthError),
                }
            }
            None => Err(GenericError::PasswordNotSetError),
        },
        Err(_) => Err(GenericError::UserNotExistError),
    }
}

#[post("/register", data = "<register_data>")]
pub async fn register(
    db: MainDatabaseConnection,
    register_data: Json<ClientUsernamePasswordData>,
    cookies: &CookieJar<'_>,
) -> GenericResult<UserLoggedInDigest> {
    let username = register_data.username.to_owned();
    match db
        .run(move |c| {
            database::users::table
                .filter(database::users::username.eq_all(username))
                .get_result::<User>(c)
        })
        .await
    {
        Ok(_) => Err(GenericError::UserAlreadyExistsError),
        Err(_) => {
            let password_hashed = argon2::hash_encoded(
                &register_data.password.as_bytes(),
                USER_AUTH_SALT.as_bytes(),
                &USER_AUTH_ARGON2_CONFIG,
            )
            .map_err(|_| GenericError::ServerInternalError)?;
            let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
            let new_user = NewUserData {
                username: Some(register_data.username.to_owned()),
                user_role: RoleEnum::User,
                password_hashed: Some(password_hashed),
                phone_number: None,
                sign_up_time: current_timestamp,
                wechat_id: None,
            };
            match db
                .run(move |c| {
                    diesel::insert_into(database::users::table)
                        .values(new_user)
                        .get_result::<User>(c)
                })
                .await
            {
                Err(_) => Err(GenericError::ServerInternalError),
                Ok(inserted_user) => {
                    match gen_token_cookie(inserted_user.id, inserted_user.user_role) {
                        Ok(token_cookie) => {
                            cookies.add(token_cookie);
                            SuccessResponse::build(UserLoggedInDigest {
                                user_role: inserted_user.user_role,
                                username: inserted_user.username,
                            })
                        }
                        Err(_) => Err(GenericError::ServerInternalError),
                    }
                }
            }
        }
    }
}

#[get("/get_users/<page>/<filter>")]
pub async fn get_users(
    db: MainDatabaseConnection,
    //    _admin: AdminAuth,
    page: i32,
    filter: RoleEnum,
) -> GenericResult<Vec<User>> {
    let results: Vec<User> = db
        .run(move |c| {
            database::users::table
                .filter(database::users::user_role.eq(filter))
                .order(database::users::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?;
    SuccessResponse::build(results)
}

#[get("/get_users/<page>")]
pub async fn get_all_users(
    db: MainDatabaseConnection,
    //  _admin: AdminAuth,
    page: i32,
) -> GenericResult<Vec<User>> {
    let results: Vec<User> = db
        .run(move |c| {
            database::users::table
                .order(database::users::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?;
    SuccessResponse::build(results)
}

#[post("/change_user_role", data = "<change_user_role_data>")]
pub async fn change_user_role(
    db: MainDatabaseConnection,
    change_user_role_data: Json<ClientChangeRoleData>,
    // _admin: AdminAuth,
) -> GenericResult<String> {
    match db
        .run(move |c| {
            diesel::update(database::users::table.find(change_user_role_data.user_id))
                .set(database::users::user_role.eq(change_user_role_data.new_role))
                .execute(c)
        })
        .await?
    {
        0 => Err(GenericError::ServerInternalError),
        _ => SuccessResponse::build("完成".to_string()),
    }
}

#[post("/remove_user", data = "<remove_user_data>")]
pub async fn remove_user(
    db: MainDatabaseConnection,
    remove_user_data: Json<ClientRemoveUserData>,
    _admin: AdminAuth,
) -> GenericResult<String> {
    match db
        .run(move |c| {
            diesel::delete(database::users::table.find(remove_user_data.user_id)).execute(c)
        })
        .await?
    {
        1 => SuccessResponse::build("完成".to_string()),
        _ => Err(GenericError::ServerInternalError),
    }
}

#[post("/change_password", data = "<change_password_data>")]
pub async fn change_password(
    db: MainDatabaseConnection,
    change_password_data: Json<ClientUsernamePasswordData>,
    user_digest: UserDigest,
) -> GenericResult<String> {
    let target_user_id = user_digest.user_id;
    let query_result: User = db
        .run(move |c| database::users::table.find(target_user_id).get_result(c))
        .await?;
    if query_result
        .username
        .ok_or(GenericError::PermissionDeniedError)?
        == change_password_data.username
    {
        let password_hashed = argon2::hash_encoded(
            &change_password_data.password.as_bytes(),
            USER_AUTH_SALT.as_bytes(),
            &USER_AUTH_ARGON2_CONFIG,
        )
        .map_err(|_| GenericError::ServerInternalError)?;
        match db
            .run(move |c| {
                diesel::update(database::users::table.find(target_user_id))
                    .set(database::users::password_hashed.eq(password_hashed))
                    .execute(c)
            })
            .await?
        {
            1 => SuccessResponse::build("完成".to_string()),
            _ => Err(GenericError::ServerInternalError),
        }
    } else {
        Err(GenericError::PermissionDeniedError)
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> GenericResult<String> {
    if let Some(_) = cookies.get("token") {
        cookies.remove(Cookie::named("token"));
    }
    SuccessResponse::build("完成".to_string())
}

#[post("/wechat_login", data = "<wechat_login_data>")]
pub async fn wechat_login(
    db: MainDatabaseConnection,
    wechat_login_data: Json<ClientWechatLoginData>,
    cookies: &CookieJar<'_>,
) -> GenericResult<UserLoggedInDigest> {
    let parsed_open_id_reponse: WechatOpenIdResponse = isahc::get_async(format!(
        "https://api.weixin.qq.com/sns/oauth2/access_token?\
    appid={}&secret={}&code={}&grant_type=authorization_code\
    ",
        *WECHAT_APPID, *WECHAT_APPSECRET, &wechat_login_data.code
    ))
    .await
    .map_err(|error| {
        error!("获取OpenId时出错：{:?}", error);
        GenericError::GetWechatOpenIdError
    })?
    .json()
    .await
    .map_err(|error| {
        error!("获取OpenId时出错：{:?}", error);
        GenericError::GetWechatOpenIdError
    })?;
    let openid = parsed_open_id_reponse.openid.to_owned();
    match db
        .run(move |c| {
            database::users::table
                .filter(database::users::wechat_id.eq_all(Some(openid)))
                .get_result::<User>(c)
                .optional()
        })
        .await?
    {
        Some(parsed_user) => match gen_token_cookie(parsed_user.id, parsed_user.user_role) {
            Ok(token_cookie) => {
                cookies.add(token_cookie);
                SuccessResponse::build(UserLoggedInDigest {
                    user_role: parsed_user.user_role,
                    username: parsed_user.username,
                })
            }
            Err(_) => Err(GenericError::ServerInternalError),
        },
        None => {
            let parsed_userinfo_reponse: WechatUserinfoResponse = isahc::get_async(format!(
                "https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}&lang=zh_CN",
                parsed_open_id_reponse.access_token, parsed_open_id_reponse.openid
            ))
            .await
            .map_err(|error| {
                error!("获取Userinfo时出错：{:?}", error);
                GenericError::GetWechatUserinfoError
            })?
            .json()
            .await
            .map_err(|error| {
                error!("获取Userinfo时出错：{:?}", error);
                GenericError::GetWechatUserinfoError
            })?;
            let current_time = Utc::now().naive_utc();
            let new_user = NewUserData {
                password_hashed: None,
                phone_number: None,
                sign_up_time: current_time,
                user_role: RoleEnum::User,
                username: Some(parsed_userinfo_reponse.nickname),
                wechat_id: Some(parsed_open_id_reponse.openid),
            };
            let insert_result: User = db
                .run(move |c| {
                    diesel::insert_into(database::users::table)
                        .values(new_user)
                        .get_result(c)
                })
                .await?;
            match gen_token_cookie(insert_result.id, insert_result.user_role) {
                Ok(token_cookie) => {
                    cookies.add(token_cookie);
                    SuccessResponse::build(UserLoggedInDigest {
                        username: insert_result.username,
                        user_role: insert_result.user_role,
                    })
                }
                Err(_) => Err(GenericError::ServerInternalError),
            }
        }
    }
}

#[get("/get_statistics")]
pub async fn get_user_statistics(
    db: MainDatabaseConnection,
    _admin: AdminAuth,
) -> GenericResult<UserStatistics> {
    let total = db
        .run(|c| database::users::table.count().get_result(c))
        .await?;
    let admin = db
        .run(|c| {
            database::users::table
                .filter(database::users::user_role.eq_all(RoleEnum::Admin))
                .count()
                .get_result(c)
        })
        .await?;
    let user = db
        .run(|c| {
            database::users::table
                .filter(database::users::user_role.eq_all(RoleEnum::User))
                .count()
                .get_result(c)
        })
        .await?;
    let staff = db
        .run(|c| {
            database::users::table
                .filter(database::users::user_role.eq_all(RoleEnum::Staff))
                .count()
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(UserStatistics {
        total,
        admin,
        staff,
        user,
    })
}
