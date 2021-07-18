use chrono::NaiveDateTime;

use serde::{self, Deserialize, Serialize};

use crate::database::*;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub submit_time: NaiveDateTime,
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
