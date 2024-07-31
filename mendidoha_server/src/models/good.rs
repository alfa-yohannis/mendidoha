use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::goods;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Goods {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub currency: String,
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>, 
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>, 
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = goods)]
pub struct NewGoods<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub currency: &'a str,
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<&'a str>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub created_by: Option<&'a str>,
    pub updated_by: Option<&'a str>,
}
