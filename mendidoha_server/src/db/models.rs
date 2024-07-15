use diesel::Queryable;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Customer
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Customer {
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

// Good
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Good {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub price: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// PurchaseOrderDetail
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PurchaseOrderDetail {
    pub id: i32,
    pub code: String,
    pub order_id: i32,
    pub item_id: i32,
    pub item_type: String,
    pub quantity: i32,
    pub unit_price: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// PurchaseOrder
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PurchaseOrder {
    pub id: i32,
    pub code: String,
    pub order_date: chrono::NaiveDate,
    pub supplier_id: i32,
    pub total_amount: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// SalesOrderDetail
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SalesOrderDetail {
    pub id: i32,
    pub code: String,
    pub order_id: i32,
    pub item_id: i32,
    pub item_type: String,
    pub quantity: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// SalesOrder
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct SalesOrder {
    pub id: i32,
    pub code: String,
    pub order_date: chrono::NaiveDate,
    pub customer_id: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// Service
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Service {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub rate: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

// Supplier
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

// TestTable
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct TestTable {
    pub id: i32,
    pub name: Option<String>,
}

// User
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
