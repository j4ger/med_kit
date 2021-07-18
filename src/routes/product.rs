use crate::auxiliary::{GenericError, GenericResult, ProductBarcode, SuccessResponse};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use rocket::serde::json::Json;

#[post("/init_product", data = "<product_data>")]
pub async fn init_product(
    db: MainDatabaseConnection,
    product_data: Json<ClientProductData>,
) -> GenericResult<String> {
    let product_barcode = product_data.product_barcode.clone();
    let current_result: i64 = db
        .run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(product_barcode))
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
            diesel::insert_into(database::products::table)
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
pub async fn get_product_digest(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<ProductDigest> {
    let barcode_input = product_barcode.inner().to_owned();
    let result: ProductDigest = db
        .run(|c| {
            database::products::table
                .select((
                    database::products::id,
                    database::products::product_barcode,
                    database::products::init_time,
                    database::products::current_stage,
                    database::products::report_id,
                ))
                .filter(database::products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(result)
}
