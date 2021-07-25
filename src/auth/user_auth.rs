use argon2;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::{Cookie, SameSite, Status};
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};
use serde::{Deserialize, Serialize};
use std::{env, fs};
use time;

use crate::auxiliary::GenericError;
use crate::models::RoleEnum;

lazy_static! {
    static ref ENCODING_KEY_FILE: Vec<u8> = read_key_from_config("USER_AUTH_ENCODING_KEY");
    static ref DECODING_KEY_FILE: Vec<u8> = read_key_from_config("USER_AUTH_DECODING_KEY");
    static ref USER_AUTH_ENCODING_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(&ENCODING_KEY_FILE).expect("EncodingKey加载失败");
    pub static ref USER_AUTH_DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_rsa_pem(&DECODING_KEY_FILE).expect("DecodingKey加载失败");
    pub static ref USER_AUTH_VALIDATION: Validation = Validation::new(Algorithm::RS256);
    static ref USER_AUTH_HEADER: Header = Header::new(Algorithm::RS256);
    pub static ref USER_AUTH_SALT: String = env::var("USER_AUTH_SALT").expect("未设置Salt");
    pub static ref USER_AUTH_ARGON2_CONFIG: argon2::Config<'static> = argon2::Config::default();
}

fn read_key_from_config(config_key: &str) -> Vec<u8> {
    let path = env::var(config_key).expect(&format!("配置项{}未设置", config_key));
    fs::read(&path).expect(&format!("文件{}不存在", &path))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,

    pub user_id: i32,
    pub user_role: RoleEnum,
}

pub struct UserDigest {
    pub user_id: i32,
    pub user_role: RoleEnum,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserDigest {
    type Error = GenericError;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(token_string) => match token_string.starts_with("Bearer") {
                true => {
                    match decode::<TokenClaims>(
                        &token_string[7..],
                        &USER_AUTH_DECODING_KEY,
                        &USER_AUTH_VALIDATION,
                    ) {
                        Ok(decoded_claims) => Outcome::Success(UserDigest {
                            user_id: decoded_claims.claims.user_id,
                            user_role: decoded_claims.claims.user_role,
                        }),
                        Err(_) => Outcome::Failure((Status::Unauthorized, GenericError::AuthError)),
                    }
                }
                false => Outcome::Failure((Status::Unauthorized, GenericError::TokenError)),
            },
            None => match request.cookies().get("token") {
                Some(token_cookie) => {
                    match decode::<TokenClaims>(
                        token_cookie.value(),
                        &USER_AUTH_DECODING_KEY,
                        &USER_AUTH_VALIDATION,
                    ) {
                        Ok(decoded_claims) => Outcome::Success(UserDigest {
                            user_id: decoded_claims.claims.user_id,
                            user_role: decoded_claims.claims.user_role,
                        }),
                        Err(_) => Outcome::Failure((Status::Unauthorized, GenericError::AuthError)),
                    }
                }
                None => Outcome::Failure((Status::Unauthorized, GenericError::AuthError)),
            },
        }
    }
}

pub fn gen_token_cookie<'a>(user_id: i32, user_role: RoleEnum) -> Result<Cookie<'a>, GenericError> {
    let expiration_datetime = Utc::now() + Duration::weeks(1);
    let new_claims = TokenClaims {
        exp: expiration_datetime,
        user_id,
        user_role,
    };
    let token = encode(&USER_AUTH_HEADER, &new_claims, &USER_AUTH_ENCODING_KEY)
        .map_err(|_| GenericError::TokenError)?;
    //TODO: Cookie options
    let output_cookie = Cookie::build("token", token)
        .expires(time::OffsetDateTime::now_utc() + time::Duration::weeks(1))
       // .http_only(true)
     //   .secure(true)
   //     .domain(env::var("COOKIE_DOMAIN").expect("未设置COOKIE_DOMAIN"))
 //       .same_site(SameSite::Lax)
        .finish();
    Ok(output_cookie)
}

// Source: https://github.com/Keats/jsonwebtoken/blob/master/examples/custom_chrono.rs
mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
