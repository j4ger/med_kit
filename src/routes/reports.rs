use crate::auth::StaffAuth;
use crate::auxiliary::{GenericError, GenericResult, SuccessResponse, UuidWrapper};
use crate::database::{self, MainDatabaseConnection};
use crate::models::*;

use diesel::prelude::*;

use chrono::prelude::*;

use rocket::data::ToByteUnit;
use rocket::Data;

use rocket::serde::json::Json;
use uuid::Uuid;

use std::env;

#[post("/upload_report", data = "<raw_data>")]
pub async fn upload_report(
    db: MainDatabaseConnection,
    raw_data: Data<'_>,
    staff: StaffAuth,
) -> GenericResult<String> {
    let filename = Uuid::new_v4().to_string();
    let base = env::var("REPORT_PATH").expect("未设置REPORT_PATH");
    let mut full_path = env::current_dir().expect("工作路径获取失败");
    full_path.push(base);
    full_path.push(&filename);
    let download_url_base = env::var("DOWNLOAD_URL_BASE").expect("未设置DOWNLOAD_URL_BASE");
    let download_url = download_url_base + &filename;
    raw_data
        .open(20.megabytes())
        .into_file(full_path)
        .await
        .map_err(|_| GenericError::ServerInternalError)?;
    let current_timestamp = Utc::now().naive_utc();
    let new_report = NewReport {
        download_url: download_url.to_owned(),
        filename: Some(filename),
        upload_time: current_timestamp,
        uploader_id: staff.user_id,
    };
    match db
        .run(move |c| {
            diesel::insert_into(database::reports::table)
                .values(new_report)
                .execute(c)
        })
        .await?
    {
        1 => SuccessResponse::build(download_url),
        _ => Err(GenericError::ServerInternalError),
    }
}

#[get("/get_reports/<page>/<uploader_id>")]
pub async fn get_filtered_reports(
    db: MainDatabaseConnection,
    page: i32,
    uploader_id: i32,
) -> GenericResult<Vec<Report>> {
    SuccessResponse::build(
        db.run(move |c| {
            database::reports::table
                .filter(database::reports::uploader_id.eq_all(uploader_id))
                .order(database::reports::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?,
    )
}

#[get("/get_reports/<page>")]
pub async fn get_reports(db: MainDatabaseConnection, page: i32) -> GenericResult<Vec<Report>> {
    SuccessResponse::build(
        db.run(move |c| {
            database::reports::table
                .order(database::reports::id)
                .limit(10)
                .offset((page * 10) as i64)
                .get_results(c)
        })
        .await?,
    )
}

#[post("/remove_report", data = "<remove_report_data>")]
pub async fn remove_report(
    db: MainDatabaseConnection,
    remove_report_data: Json<ClientRemoveReportData>,
    _staff: StaffAuth,
) -> GenericResult<String> {
    match db
        .run(move |c| {
            diesel::delete(database::reports::table.find(remove_report_data.report_id)).execute(c)
        })
        .await?
    {
        1 => SuccessResponse::build("完成".to_string()),
        _ => Err(GenericError::InvalidInputError),
    }
}

#[get("/get_report/<report_id>")]
pub async fn get_report(
    db: MainDatabaseConnection,
    report_id: UuidWrapper,
) -> GenericResult<String> {
    SuccessResponse::build(
        db.run(move |c| {
            database::reports::table
                .find::<Uuid>(report_id.into())
                .get_result::<Report>(c)
        })
        .await?
        .download_url,
    )
}
