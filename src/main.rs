mod db;
mod profile;
mod routes;

use actix_web::{App, HttpServer};
use db::DBService;
use routes::{fetch, new_prod, submit};

pub struct AppState {
    user: DBService,
    profile: DBService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_service = DBService::by_collection_name("user").await;
    let profile_service = DBService::by_collection_name("profile").await;
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                user: user_service.clone(),
                profile: profile_service.clone(),
            })
            .service(new_prod)
            .service(fetch)
            .service(submit)
    })
    .bind("127.0.0.1:1146")?
    .run()
    .await
}
