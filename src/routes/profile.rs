use crate::auth::UserDigest;
use crate::auxiliary::{GenericError, GenericResult, ProductBarcode, SuccessResponse};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use rocket::serde::json::Json;

#[post("/submit_profile/<product_barcode>", data = "<profile_data>")]
pub async fn submit_profile_then_update(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
    profile_data: Json<NewProfileData>,
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

#[post("/submit_profile", data = "<profile_data>")]
pub async fn submit_profile(
    db: MainDatabaseConnection,
    user_digest: UserDigest,
    profile_data: Json<NewProfileData>,
) -> GenericResult<String> {
    let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    //TODO: form validation
    let mut new_profile = profile_data.into_inner();
    new_profile.user_id = user_digest.user_id;
    new_profile.submit_time = current_timestamp;
    match db
        .run(move |c| {
            diesel::insert_into(database::profiles::table)
                .values(new_profile)
                .execute(c)
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

#[get("/get_profile_by_user")]
pub async fn get_profile_by_user(
    db: MainDatabaseConnection,
    user_digest: UserDigest,
) -> GenericResult<Vec<Profile>> {
    SuccessResponse::build(
        db.run(move |c| {
            database::profiles::table
                .filter(database::profiles::user_id.eq_all(user_digest.user_id))
                .get_results(c)
        })
        .await?,
    )
}

#[post("/bind_profile/<product_barcode>", data = "<bind_profile_data>")]
pub async fn bind_profile(
    db: MainDatabaseConnection,
    product_barcode: ProductBarcode<'_>,
    bind_profile_data: Json<BindProfileData>,
    user_digest: UserDigest,
) -> GenericResult<String> {
    let barcode_input = product_barcode.inner().to_owned();
    let barcode_input_clone = barcode_input.clone();
    let profile_id = bind_profile_data.profile_id;
    let query_result: Product = db
        .run(move |c| {
            database::products::table
                .filter(database::products::product_barcode.eq_all(barcode_input))
                .get_result(c)
        })
        .await?;
    if query_result.current_stage != StageEnum::Initialized {
        Err(GenericError::ProductReuseError)
    } else {
        let profile_result: Profile = db
            .run(move |c| database::profiles::table.find(&profile_id).get_result(c))
            .await?;
        if !(profile_result.user_id == user_digest.user_id
            || user_digest.user_role == RoleEnum::Admin
            || user_digest.user_role == RoleEnum::Staff)
        {
            Err(GenericError::PermissionDeniedError)
        } else {
            match db
                .run(move |c| {
                    diesel::update(
                        database::products::table.filter(
                            database::products::product_barcode.eq_all(barcode_input_clone),
                        ),
                    )
                    .set((
                        database::products::profile_id.eq_all(Some(profile_id)),
                        database::products::current_stage.eq_all(StageEnum::Submitted),
                    ))
                    .execute(c)
                })
                .await?
            {
                1 => SuccessResponse::build("成功".to_string()),
                _ => Err(GenericError::ServerInternalError),
            }
        }
    }
}
