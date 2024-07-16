use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};
use crate::schema::suppliers;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Supplier {
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = suppliers)]
pub struct NewSupplier<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub created_by: Option<&'a str>,
    pub updated_by: Option<&'a str>,
}
