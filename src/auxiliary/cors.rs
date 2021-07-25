use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use std::env;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS policy specifier",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let cors_domain = env::var("CORS_DOMAIN").expect("未设置CORS_DOMAIN");
        response.set_header(Header::new("Access-Control-Allow-Origin", cors_domain));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
