use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub code: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub code: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub first_name: &'a str,
    pub middle_name: Option<&'a str>,
    pub last_name: &'a str,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub created_by: Option<&'a str>,
    pub updated_by: Option<&'a str>,
}
