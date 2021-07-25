use crate::auth::{StaffAuth, UserDigest};
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
    user_digest: UserDigest,
) -> GenericResult<Product> {
    let barcode_input = product_barcode.inner().to_owned();
    let result: Product = db
        .run(|c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(barcode_input))
                .limit(1)
                .get_result(c)
        })
        .await?;
    match result.current_stage {
        StageEnum::Initialized => SuccessResponse::build(result),
        _ => match user_digest.user_role {
            RoleEnum::User => {
                let profile_id = result.profile_id.ok_or(GenericError::ServerInternalError)?;
                let profile_query_result: Profile = db
                    .run(move |c| database::profiles::table.find(profile_id).get_result(c))
                    .await?;
                if profile_query_result.user_id == user_digest.user_id {
                    SuccessResponse::build(result)
                } else {
                    Err(GenericError::PermissionDeniedError)
                }
            }
            _ => SuccessResponse::build(result),
        },
    }
}

#[get("/get_products/<page>/<filter>")]
pub async fn get_products(
    db: MainDatabaseConnection,
    page: i32,
    filter: StageEnum,
    _staff: StaffAuth,
) -> GenericResult<Vec<Product>> {
    SuccessResponse::build(
        db.run(move |c| {
            database::products::table
                .filter(database::products::current_stage.eq(filter))
                .order(database::products::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?,
    )
}

#[get("/get_products/<page>")]
pub async fn get_filtered_products(
    db: MainDatabaseConnection,
    page: i32,
    _staff: StaffAuth,
) -> GenericResult<Vec<Product>> {
    SuccessResponse::build(
        db.run(move |c| {
            database::products::table
                .order(database::products::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?,
    )
}
