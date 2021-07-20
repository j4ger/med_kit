mod auth;
mod auxiliary;
mod database;
mod models;
mod routes;

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

use fern;

use chrono;

use log;

use std::env;

use crate::auxiliary::CORS;
use crate::database::MainDatabaseConnection;
use crate::routes::*;

#[launch]
fn launch_rocket() -> _ {
    dotenv().ok();
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(
            fern::log_file(env::var("LOG_FILE").expect("未设置LOG_FILE")).expect("无法打开log文件"),
        )
        .apply()
        .expect("log引擎初始化错误");
    rocket::build()
        .mount("/api/user", user_routes())
        .mount("/api/product", product_routes())
        .mount("/api/profile", profile_routes())
        .mount("/api/report", report_routes())
        .attach(MainDatabaseConnection::fairing())
        .register("/api", api_error_catchers())
        //TODO:CORS
        .attach(CORS)
}

//TODO: logging
//TODO: database constrains
