mod auth;
mod config;
mod db;
mod profile;
mod routes;
mod user;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use auth::JWTAuthService;
use db::DBService;
use routes::{
    get_profile, init_product, login, new_product, query_profile, raw_register, submit_profile,
    verify_token,
};

pub struct AppState {
    user: DBService,
    profile: DBService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let user_service = DBService::by_collection_name("user").await;
    let profile_service = DBService::by_collection_name("profile").await;
    HttpServer::new(move || {
        //TODO: CORS config
        let any_cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        let strict_cors = Cors::default()
            .allowed_origin("https://medkit.j4ger.cn")
            .allowed_origin("https://servicewechat.com")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://172.31.38.196:3000")
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i Response:%s %b"))
            //TODO: cors
            .wrap(strict_cors)
            .data(AppState {
                user: user_service.clone(),
                profile: profile_service.clone(),
            })
            .service(new_product)
            .service(init_product)
            .service(get_profile)
            .service(verify_token)
            .service(raw_register)
            .service(query_profile)
            .service(submit_profile)
            .service(login)
    })
    .bind(("0.0.0.0", 1146))?
    .run()
    .await
}
