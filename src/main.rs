mod auxiliary;
mod database;
mod models;
mod routes;
mod user;

#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate lazy_static;

use dotenv::dotenv;

use crate::auxiliary::CORS;
use crate::database::MainDatabaseConnection;
use crate::routes::*;

#[launch]
fn launch_rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/api/user", user_routes())
        .mount("/api/product", product_routes())
        .mount("/api/profile", profile_routes())
        .attach(MainDatabaseConnection::fairing())
        .register("/api", api_error_catchers())
        //TODO:CORS
        .attach(CORS)
}

//TODO: logging
