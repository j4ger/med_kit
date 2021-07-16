mod database_connection;
mod models;
mod product_barcode;
mod responses;
mod schema;

#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

use responses::GenericError;
use rocket::serde::json::Json;

use dotenv::dotenv;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use crate::database_connection::MainDatabaseConnection;
use crate::models::*;
use crate::product_barcode::ProductBarcode;
use crate::responses::{GenericResult, SuccessResponse};
use crate::schema::*;

#[launch]
fn launch_rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount(
            "/api",
            routes![
                get_all_profiles,
                init_product,
                get_product_digest,
                submit_profile,
                get_profile
            ],
        )
        .attach(MainDatabaseConnection::fairing())
}

#[get("/all_profiles")]
async fn get_all_profiles(db: MainDatabaseConnection) -> GenericResult<Vec<models::Profile>> {
    let results = db.run(|c| profiles::table.load::<Profile>(c)).await?;
    SuccessResponse::build(results)
}

#[post("/init_product", data = "<product_data>")]
async fn init_product(
    db: MainDatabaseConnection,
    product_data: Json<ClientProductData>,
) -> GenericResult<String> {
    let product_barcode = product_data.product_barcode.clone();
    let current_result: i64 = db
        .run(move |c| {
            products::table
                .filter(products::product_barcode.eq_all(product_barcode))
                .count()
                .get_result(c)
        })
        .await?;
    if current_result == 1 {
        return Err(GenericError::ProductDuplicateError);
    }

    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    let new_product = NewProductData {
        product_barcode: product_data.product_barcode.clone(),
        init_time: current_timestamp,
    };
    match db
        .run(|c| {
            diesel::insert_into(products::table)
                .values(new_product)
                .execute(c)
        })
        .await?
    {
        0 => Err(GenericError::ProductDuplicateError),
        _ => SuccessResponse::build("成功".to_string()),
    }
}

#[get("/get_product_digest/<product_barcode>")]
async fn get_product_digest(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<ProductDigest> {
    let barcode_input = product_barcode.inner().to_owned();
    let result: ProductDigest = db
        .run(|c| {
            products::table
                .select((
                    products::id,
                    products::product_barcode,
                    products::init_time,
                    products::current_stage,
                    products::report_id,
                ))
                .filter(products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(result)
}

#[post("/submit_profile/<product_barcode>", data = "<profile_data>")]
async fn submit_profile(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
    profile_data: Json<ClientProfileData>,
) -> GenericResult<String> {
    let barcode_input = product_barcode.inner().to_owned();
    let query_result: ProductDigest = db
        .run(|c| {
            products::table
                .select((
                    products::id,
                    products::product_barcode,
                    products::init_time,
                    products::current_stage,
                    products::report_id,
                ))
                .filter(products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    if query_result.current_stage != StageEnum::Initialized {
        return Err(GenericError::ProductReuseError);
    }
    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    //TODO: form validation
    let new_profile = NewProfileData {
        //TODO: user_id
        user_id: profile_data.user_id,
        submit_time: current_timestamp,
    };
    match db
        .run(move |c| {
            diesel::insert_into(profiles::table)
                .values(new_profile)
                .get_result(c)
                .and_then(|insert_result: Profile| {
                    let update_set = UpdateProductAfterSubmittion {
                        profile_id: Some(insert_result.id),
                        current_stage: StageEnum::Submitted,
                    };
                    diesel::update(products::table.find(query_result.id))
                        .set(update_set)
                        .execute(c)
                })
        })
        .await?
    {
        0 => Err(GenericError::ProductReuseError),
        _ => SuccessResponse::build("提交成功".to_string()),
    }
}

#[get("/get_profile/<profile_id>")]
async fn get_profile(db: MainDatabaseConnection, profile_id: i32) -> GenericResult<Profile> {
    let result: Profile = db
        .run(move |c| profiles::table.find(profile_id).get_result(c))
        .await?;
    SuccessResponse::build(result)
}
