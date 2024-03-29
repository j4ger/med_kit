use jsonwebtoken::decode;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

use crate::auth::{TokenClaims, USER_AUTH_DECODING_KEY, USER_AUTH_VALIDATION};
use crate::auxiliary::GenericError;
use crate::models::RoleEnum;

pub struct StaffAuth {
    pub user_id: i32,
    pub user_role: RoleEnum,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaffAuth {
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
                        Ok(decoded_claims) => match decoded_claims.claims.user_role {
                            RoleEnum::User => Outcome::Failure((
                                Status::Forbidden,
                                GenericError::PermissionDeniedError,
                            )),
                            _ => Outcome::Success(StaffAuth {
                                user_id: decoded_claims.claims.user_id,
                                user_role: decoded_claims.claims.user_role,
                            }),
                        },
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
                        Ok(decoded_claims) => match decoded_claims.claims.user_role {
                            RoleEnum::User => Outcome::Failure((
                                Status::Forbidden,
                                GenericError::PermissionDeniedError,
                            )),
                            _ => Outcome::Success(StaffAuth {
                                user_id: decoded_claims.claims.user_id,
                                user_role: decoded_claims.claims.user_role,
                            }),
                        },
                        Err(error) => {
                            info!("{:?}", error);
                            Outcome::Failure((Status::Unauthorized, GenericError::AuthError))
                        }
                    }
                }
                None => {
                    info!("No token was found.");
                    Outcome::Failure((Status::Unauthorized, GenericError::AuthError))
                }
            },
        }
    }
}
