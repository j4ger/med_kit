use crate::{config::get_config, routes::gen_std_json_response};
use actix_web::{
    dev, error::ErrorUnauthorized, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse,
};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub struct JWTAuthService;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub usr: String,
}

#[derive(Deserialize, Serialize)]
struct VoidData {}

impl FromRequest for JWTAuthService {
    type Error = HttpResponse;
    type Future = Ready<Result<JWTAuthService, HttpResponse>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.cookie("JWT") {
            Some(result) => {
                let token = result.value();
                let secret = get_config("JWTSECRET");
                let key = DecodingKey::from_secret(secret.as_bytes());
                match decode::<Claims>(&token, &key, &Validation::new(Algorithm::HS256)) {
                    Ok(_) => ok(JWTAuthService),
                    Err(_) => err(gen_std_json_response::<VoidData>(
                        None,
                        Some("认证无效！".to_string()),
                    )),
                }
            }
            None => err(gen_std_json_response::<VoidData>(
                None,
                Some("未包含认证信息！".to_string()),
            )),
        }
    }
}
