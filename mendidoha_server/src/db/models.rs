// use diesel::prelude::*;
use diesel::Queryable;
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String
}
