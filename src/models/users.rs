use chrono::NaiveDateTime;

use rocket::request::FromParam;

use serde::{self, Deserialize, Serialize};

use crate::auxiliary::GenericError;
use crate::database::*;

#[derive(DbEnum, Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
#[DieselType = "Role"]
#[DbValueStyle = "PascalCase"]
pub enum RoleEnum {
    User,
    Staff,
    Admin,
}

impl<'a> FromParam<'a> for RoleEnum {
    type Error = GenericError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param.to_lowercase().as_ref() {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            "staff" => Ok(Self::Staff),
            _ => Err(GenericError::InvalidInputError),
        }
    }
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
pub struct ClientUsernamePasswordData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ClientChangeRoleData {
    pub user_id: i32,
    pub new_role: RoleEnum,
}

#[derive(Deserialize)]
pub struct ClientRemoveUserData {
    pub user_id: i32,
}
