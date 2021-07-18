use crate::auxiliary::{GenericError, GenericResult, SuccessResponse};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;
use crate::user::{gen_token_cookie, UserDigest, USER_AUTH_ARGON2_CONFIG, USER_AUTH_SALT};

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use rocket::http::CookieJar;
use rocket::serde::json::Json;

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
    login_data: Json<ClientLoginData>,
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
    register_data: Json<ClientRegisterData>,
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
