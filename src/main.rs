mod database_connection;
mod models;
mod product_id;
mod responses;
mod schema;

#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;

use diesel::prelude::*;

use crate::database_connection::MainDatabaseConnection;
use crate::models::*;
use crate::responses::{GenericResult, SuccessResponse};
use crate::schema::profiles;

#[launch]
fn launch_rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/api", routes![get_all_profiles])
        .attach(MainDatabaseConnection::fairing())
}

#[get("/all_profiles")]
async fn get_all_profiles(db: MainDatabaseConnection) -> GenericResult<Vec<models::Profile>> {
    let results = db.run(|c| profiles::table.load::<Profile>(c)).await?;
    SuccessResponse::build(results)
}
