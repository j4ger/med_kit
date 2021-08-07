use chrono::NaiveDateTime;

use serde::{self, Deserialize, Serialize};

use uuid::Uuid;

use crate::database::*;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Report {
    pub id: Uuid,
    pub uploader_id: i32,
    pub filename: Option<String>,
    pub download_url: String,
    pub upload_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[table_name = "reports"]
pub struct NewReport {
    pub uploader_id: i32,
    pub filename: Option<String>,
    pub download_url: String,
    pub upload_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ClientRemoveReportData {
    pub report_id: Uuid,
}

#[derive(Deserialize)]
pub struct PublishReportData {
    pub product_barcode: String,
    pub filename: Option<String>,
    pub download_url: Option<String>,
}
