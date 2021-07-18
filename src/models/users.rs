use chrono::NaiveDateTime;
use serde;
use serde::{Deserialize, Serialize};

use crate::database::*;

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
#[DieselType = "Role"]
#[DbValueStyle = "PascalCase"]
pub enum RoleEnum {
    User,
    Staff,
    Admin,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub wechat_id: Option<String>,
    pub user_role: RoleEnum,
    pub password_hashed: Option<String>,
    pub phone_number: Option<i32>,
    pub sign_up_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Clone, Debug)]
#[table_name = "users"]
pub struct NewUserData {
    pub username: Option<String>,
    pub wechat_id: Option<String>,
    pub user_role: RoleEnum,
    pub password_hashed: Option<String>,
    pub phone_number: Option<i32>,
    pub sign_up_time: NaiveDateTime,
}

#[derive(Serialize)]
pub struct UserLoggedInDigest {
    pub username: Option<String>,
    pub user_role: RoleEnum,
}

#[derive(Deserialize)]
pub struct ClientLoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ClientRegisterData {
    pub username: String,
    pub password: String,
}
