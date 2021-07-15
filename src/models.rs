use chrono::NaiveDateTime;
use serde;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone, PartialEq)]
#[DieselType = "Stage"]
#[DbValueStyle = "PascalCase"]
pub enum StageEnum {
    Initialized,
    Submitted,
    Finished,
}

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone, PartialEq)]
#[DieselType = "Role"]
#[DbValueStyle = "PascalCase"]
pub enum RoleEnum {
    User,
    Staff,
    Admin,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub submit_time: NaiveDateTime,
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

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub wechat_id: Option<String>,
    pub user_role: RoleEnum,
    pub password_hashed: Option<String>,
    pub phone_number: Option<i32>,
    pub sign_up_time: NaiveDateTime,
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

#[derive(Deserialize)]
pub struct ClientProfileData {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[table_name = "profiles"]
pub struct NewProfileData {
    pub user_id: i32,
    pub submit_time: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "products"]
pub struct UpdateProductAfterSubmittion {
    pub profile_id: Option<i32>,
    pub current_stage: StageEnum,
}
