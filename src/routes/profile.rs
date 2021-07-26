use crate::auth::UserDigest;
use crate::auxiliary::{GenericError, GenericResult, ProductBarcode, SuccessResponse};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use rocket::serde::json::Json;

#[post("/submit_profile/<product_barcode>", data = "<profile_data>")]
pub async fn submit_profile(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
    profile_data: Json<Profile>,
    user_digest: UserDigest,
) -> GenericResult<String> {
    let barcode_input = product_barcode.inner().to_owned();
    let query_result: ProductDigest = db
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
    if query_result.current_stage != StageEnum::Initialized {
        return Err(GenericError::ProductReuseError);
    }
    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    //TODO: form validation
    let mut new_profile = profile_data.into_inner();
    new_profile.user_id = user_digest.user_id;
    new_profile.submit_time = current_timestamp;
    new_profile.id = 0;
    match db
        .run(move |c| {
            diesel::insert_into(database::profiles::table)
                .values(new_profile)
                .get_result(c)
                .and_then(|insert_result: Profile| {
                    let update_set = UpdateProductAfterSubmission {
                        profile_id: Some(insert_result.id),
                        current_stage: StageEnum::Submitted,
                    };
                    diesel::update(database::products::table.find(query_result.id))
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
pub async fn get_profile(db: MainDatabaseConnection, profile_id: i32) -> GenericResult<Profile> {
    let result: Profile = db
        .run(move |c| database::profiles::table.find(profile_id).get_result(c))
        .await?;
    SuccessResponse::build(result)
}
