use chrono::NaiveDateTime;
use serde;
use serde::{Deserialize, Serialize};

use crate::database::*;

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
#[DieselType = "Stage"]
#[DbValueStyle = "PascalCase"]
pub enum StageEnum {
    Initialized,
    Submitted,
    Finished,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub product_barcode: String,
    pub profile_id: Option<i32>,
    pub init_time: NaiveDateTime,
    pub current_stage: StageEnum,
    pub report_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct ClientProductData {
    pub product_barcode: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug)]
#[table_name = "products"]
pub struct NewProductData {
    pub product_barcode: String,
    pub init_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ProductDigest {
    #[serde(skip_serializing)]
    pub id: i32,

    pub product_barcode: String,
    pub init_time: NaiveDateTime,
    pub current_stage: StageEnum,
    pub report_id: Option<i32>,
}

#[derive(AsChangeset)]
#[table_name = "products"]
pub struct UpdateProductAfterSubmission {
    pub profile_id: Option<i32>,
    pub current_stage: StageEnum,
}
