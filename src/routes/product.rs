use crate::auth::{StaffAuth, UserDigest};
use crate::auxiliary::{
    GenericError, GenericResult, ProductBarcode, ProductBarcodeGeneratorState, SuccessResponse,
};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use rocket::serde::json::Json;
use rocket::State;

use std::env;

lazy_static! {
    static ref QRCODE_DOMAIN_ROOT: String =
        env::var("QRCODE_ROOT_DOMAIN").expect("未设置QRCODE_ROOT_DOMAIN");
}

#[get("/init_product")]
pub async fn init_product(
    db: MainDatabaseConnection,
    _staff: StaffAuth,
    barcode_generator: &State<ProductBarcodeGeneratorState>,
) -> GenericResult<String> {
    let product_barcode = barcode_generator.get().await;
    let product_barcode_clone = product_barcode.clone();
    let result: String = format!("{}{}", *QRCODE_DOMAIN_ROOT, &product_barcode);
    let current_result: i64 = db
        .run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(product_barcode_clone))
                .count()
                .get_result(c)
        })
        .await?;
    if current_result == 1 {
        return Err(GenericError::ProductDuplicateError);
    }

    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    let new_product = NewProductData {
        product_barcode: product_barcode,
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
        _ => SuccessResponse::build(result),
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
pub async fn get_filtered_products(
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
pub async fn get_products(
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

#[get("/get_product/<product_barcode>")]
pub async fn get_product(
    db: MainDatabaseConnection,
    _staff: StaffAuth,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<Product> {
    let barcode_input = product_barcode.inner().to_owned();
    SuccessResponse::build(
        db.run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq(barcode_input))
                .get_result(c)
        })
        .await?,
    )
}

#[get("/get_profile/<product_barcode>")]
pub async fn get_profile_by_product(
    db: MainDatabaseConnection,
    _staff: StaffAuth,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<Profile> {
    let barcode_input = product_barcode.inner().to_owned();
    let query_result: Product = db
        .run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(barcode_input))
                .get_result(c)
        })
        .await?;
    let profile_id = query_result
        .profile_id
        .ok_or(GenericError::ProfileNotExistError)?;
    SuccessResponse::build(
        db.run(move |c| database::profiles::table.find(profile_id).get_result(c))
            .await?,
    )
}

#[post("/submit_sample_time/<product_barcode>", data = "<sample_time_data>")]
pub async fn submit_sample_time(
    db: MainDatabaseConnection,
    user_digest: UserDigest,
    sample_time_data: Json<SampleTimeData>,
    product_barcode: ProductBarcode<'_>,
) -> GenericResult<String> {
    let input_barcode = product_barcode.inner().to_owned();
    let query_barcode = input_barcode.clone();
    let query_result: Product = db
        .run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(query_barcode))
                .get_result(c)
        })
        .await?;
    let profile_id = query_result
        .profile_id
        .ok_or(GenericError::ProfileNotExistError)?;
    let profile_result: Profile = db
        .run(move |c| database::profiles::table.find(profile_id).get_result(c))
        .await?;
    if !(profile_result.user_id == user_digest.user_id
        || user_digest.user_role == RoleEnum::Admin
        || user_digest.user_role == RoleEnum::Staff)
    {
        Err(GenericError::PermissionDeniedError)
    } else {
        match db
            .run(move |c| {
                diesel::update(database::profiles::table.find(profile_id))
                    .set(database::profiles::sample_time.eq_all(sample_time_data.sample_time))
                    .execute(c)
            })
            .await
        {
            Ok(_) => {
                if query_result.current_stage == StageEnum::Submitted
                    || query_result.current_stage == StageEnum::Sampled
                {
                    match db
                        .run(move |c| {
                            diesel::update(
                                database::products::table
                                    .filter(database::products::product_barcode.eq(input_barcode)),
                            )
                            .set(database::products::current_stage.eq(StageEnum::Sampled))
                            .execute(c)
                        })
                        .await
                    {
                        Ok(_) => SuccessResponse::build("成功".to_string()),
                        Err(_) => Err(GenericError::ServerInternalError),
                    }
                } else {
                    SuccessResponse::build("成功".to_string())
                }
            }
            Err(_) => Err(GenericError::ServerInternalError),
        }
    }
}

#[get("/get_statistics")]
pub async fn get_product_statistics(
    db: MainDatabaseConnection,
    _staff: StaffAuth,
) -> GenericResult<ProductStatistics> {
    let total = db
        .run(|c| database::products::table.count().get_result(c))
        .await?;
    let initialized = db
        .run(|c| {
            database::products::table
                .filter(database::products::current_stage.eq_all(StageEnum::Initialized))
                .count()
                .get_result(c)
        })
        .await?;
    let submitted = db
        .run(|c| {
            database::products::table
                .filter(database::products::current_stage.eq_all(StageEnum::Submitted))
                .count()
                .get_result(c)
        })
        .await?;

    let sampled = db
        .run(|c| {
            database::products::table
                .filter(database::products::current_stage.eq_all(StageEnum::Sampled))
                .count()
                .get_result(c)
        })
        .await?;
    let finished = db
        .run(|c| {
            database::products::table
                .filter(database::products::current_stage.eq_all(StageEnum::Finished))
                .count()
                .get_result(c)
        })
        .await?;
    SuccessResponse::build(ProductStatistics {
        total,
        initialized,
        submitted,
        sampled,
        finished,
    })
}
