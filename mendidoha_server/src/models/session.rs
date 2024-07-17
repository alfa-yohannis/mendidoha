use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::sessions;


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i32,
    pub user_code: String,
    pub device_id: String,
    pub session_id: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expiry_time: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub user_code: &'a str,
    pub device_id: &'a str,
    pub session_id: &'a str,
    pub start_time: DateTime<Utc>,
    pub expiry_time: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub created_by: Option<&'a str>,
    pub updated_by: Option<&'a str>,
}
