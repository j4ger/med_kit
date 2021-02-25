mod db;
mod profile;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use db::DBService;
use routes::{fetch, init_prod, new_prod, submit};

pub struct AppState {
    user: DBService,
    profile: DBService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .wrap(cors)
            .data(AppState {
                user: user_service.clone(),
                profile: profile_service.clone(),
            })
            .service(new_prod)
            .service(init_prod)
            .service(fetch)
            .service(submit)
    })
    .bind(("0.0.0.0", 1146))?
    .run()
    .await
}
