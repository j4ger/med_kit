use std::convert::TryInto;

use chrono::NaiveDateTime;

use serde::{self, de, Deserialize, Deserializer, Serialize};

use crate::database::*;

#[derive(Serialize, Deserialize, Insertable, Clone, Debug, Queryable)]
#[table_name = "profiles"]
pub struct Profile {
    #[serde(skip_serializing_if = "i32_equals_0")]
    #[serde(default = "default_i32")]
    pub id: i32,
    #[serde(default = "default_i32")]
    pub user_id: i32,
    #[serde(default = "default_naive_date_time")]
    pub submit_time: NaiveDateTime,

    name: String,
    age: i32,
    #[serde(deserialize_with = "deserialize_i64_to_naive_date_time")]
    sample_time: NaiveDateTime,
}

fn default_i32() -> i32 {
    0
}

fn default_naive_date_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

fn i32_equals_0(source: &i32) -> bool {
    source == &0
}

fn deserialize_i64_to_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let parsed_integer = i64::deserialize(deserializer)?;
    Ok(NaiveDateTime::from_timestamp(parsed_integer, 0))
}
