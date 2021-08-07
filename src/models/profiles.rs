use chrono::NaiveDateTime;

use serde::{self, Deserialize, Deserializer, Serialize};

use crate::database::*;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub submit_time: NaiveDateTime,

    name: String,
    id_card_number: String,
    birth_date: NaiveDateTime,
    profession: String,
    address: String,
    phone: String,

    sample_time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[table_name = "profiles"]
pub struct NewProfileData {
    #[serde(default = "default_i32")]
    pub user_id: i32,
    #[serde(default = "default_naive_date_time")]
    pub submit_time: NaiveDateTime,

    name: String,
    id_card_number: String,
    #[serde(deserialize_with = "deserialize_i64_to_naive_date_time")]
    birth_date: NaiveDateTime,
    profession: String,
    address: String,
    phone: String,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[table_name = "profiles"]
pub struct SampleTimeData {
    #[serde(deserialize_with = "deserialize_i64_to_naive_date_time")]
    pub sample_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct BindProfileData {
    pub profile_id: i32,
}

fn default_i32() -> i32 {
    0
}

fn default_naive_date_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

fn deserialize_i64_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let parsed_integer = i64::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp(parsed_integer / 1000, 0))
}
