mod cors;
mod database_connection;
mod error_catchers;
mod models;
mod product_barcode;
mod responses;
mod schema;
mod user_auth;

#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate lazy_static;

use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

use dotenv::dotenv;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use argon2;

use crate::cors::CORS;
use crate::database_connection::MainDatabaseConnection;
use crate::error_catchers::default_error_catcher;
use crate::models::*;
use crate::product_barcode::ProductBarcode;
use crate::responses::{GenericError, GenericResult, SuccessResponse};
use crate::schema::*;
use crate::user_auth::{gen_token_cookie, UserDigest, USER_AUTH_ARGON2_CONFIG, USER_AUTH_SALT};

#[launch]
fn launch_rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount(
            "/api",
            routes![
                get_all_profiles,
                init_product,
                get_product_digest,
                submit_profile,
                get_profile,
                verify_login,
                login,
                register
            ],
        )
        .attach(MainDatabaseConnection::fairing())
        .register("/api", catchers![default_error_catcher])
        //TODO:CORS
        .attach(CORS)
}

#[get("/all_profiles")]
async fn get_all_profiles(db: MainDatabaseConnection) -> GenericResult<Vec<models::Profile>> {
    let results = db.run(|c| profiles::table.load::<Profile>(c)).await?;
    SuccessResponse::build(results)
}

#[post("/init_product", data = "<product_data>")]
async fn init_product(
    db: MainDatabaseConnection,
    product_data: Json<ClientProductData>,
) -> GenericResult<String> {
    let product_barcode = product_data.product_barcode.clone();
    let current_result: i64 = db
        .run(move |c| {
            products::table
                .filter(products::product_barcode.eq_all(product_barcode))
                .count()
                .get_result(c)
        })
        .await?;
    if current_result == 1 {
        return Err(GenericError::ProductDuplicateError);
    }

    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    let new_product = NewProductData {
        product_barcode: product_data.product_barcode.clone(),
        init_time: current_timestamp,
    };
    match db
        .run(|c| {
            diesel::insert_into(products::table)
                .values(new_product)
                .execute(c)
        })
        .await?
    {
        0 => Err(GenericError::ProductDuplicateError),
        _ => SuccessResponse::build("成功".to_string()),
    }
}

#[get("/get_product_digest/<product_barcode>")]
async fn get_product_digest(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<ProductDigest> {
    let barcode_input = product_barcode.inner().to_owned();
    let result: ProductDigest = db
        .run(|c| {
            products::table
                .select((
                    products::id,
                    products::product_barcode,
                    products::init_time,
                    products::current_stage,
                    products::report_id,
                ))
                .filter(products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(result)
}

#[post("/submit_profile/<product_barcode>", data = "<profile_data>")]
async fn submit_profile(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
    profile_data: Json<ClientProfileData>,
) -> GenericResult<String> {
    let barcode_input = product_barcode.inner().to_owned();
    let query_result: ProductDigest = db
        .run(|c| {
            products::table
                .select((
                    products::id,
                    products::product_barcode,
                    products::init_time,
                    products::current_stage,
                    products::report_id,
                ))
                .filter(products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    if query_result.current_stage != StageEnum::Initialized {
        return Err(GenericError::ProductReuseError);
    }
    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    //TODO: form validation
    let new_profile = NewProfileData {
        //TODO: user_id
        user_id: profile_data.user_id,
        submit_time: current_timestamp,
    };
    match db
        .run(move |c| {
            diesel::insert_into(profiles::table)
                .values(new_profile)
                .get_result(c)
                .and_then(|insert_result: Profile| {
                    let update_set = UpdateProductAfterSubmittion {
                        profile_id: Some(insert_result.id),
                        current_stage: StageEnum::Submitted,
                    };
                    diesel::update(products::table.find(query_result.id))
                        .set(update_set)
                        .execute(c)
                })
        })
        .await?
    {
        0 => Err(GenericError::ProductReuseError),
        _ => SuccessResponse::build("提交成功".to_string()),
    }
}

#[get("/get_profile/<profile_id>")]
async fn get_profile(db: MainDatabaseConnection, profile_id: i32) -> GenericResult<Profile> {
    let result: Profile = db
        .run(move |c| profiles::table.find(profile_id).get_result(c))
        .await?;
    SuccessResponse::build(result)
}

#[get("/verify_login")]
async fn verify_login(
    user_digest: UserDigest,
    db: MainDatabaseConnection,
) -> GenericResult<UserLoggedInDigest> {
    let query_result: User = db
        .run(move |c| users::table.find(user_digest.user_id).get_result(c))
        .await?;
    SuccessResponse::build(UserLoggedInDigest {
        username: query_result.username,
        user_role: query_result.user_role,
    })
}

#[post("/login", data = "<login_data>")]
async fn login(
    db: MainDatabaseConnection,
    login_data: Json<ClientLoginData>,
    cookies: &CookieJar<'_>,
) -> GenericResult<UserLoggedInDigest> {
    let username = login_data.username.to_owned();
    match db
        .run(move |c| {
            users::table
                .filter(users::username.eq_all(username))
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
async fn register(
    db: MainDatabaseConnection,
    register_data: Json<ClientRegisterData>,
    cookies: &CookieJar<'_>,
) -> GenericResult<UserLoggedInDigest> {
    let username = register_data.username.to_owned();
    match db
        .run(move |c| {
            users::table
                .filter(users::username.eq_all(username))
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
                    diesel::insert_into(users::table)
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
