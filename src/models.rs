use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use serde;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Deserialize, Serialize)]
pub enum Stage {
    Initialized,
    Submitted,
    Finished,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub submit_time: NaiveDateTime,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub product_barcode: String,
    pub profile_id: Option<i32>,
    pub init_time: NaiveDateTime,
    pub current_stage: Stage,
    pub report_id: Option<i32>,
}
