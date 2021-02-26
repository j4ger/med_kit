mod db;
mod profile;
mod routes;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use db::DBService;
use routes::{fetch_profile, init_product, new_product, submit_profile};

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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .data(AppState {
                user: user_service.clone(),
                profile: profile_service.clone(),
            })
            .data(web::JsonConfig::default().limit(65536))
            .service(new_product)
            .service(init_product)
            .service(fetch_profile)
            .service(submit_profile)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
