use rocket_sync_db_pools::{database, diesel};

#[database("med_kit")]
pub struct MainDatabaseConnection(diesel::PgConnection);
