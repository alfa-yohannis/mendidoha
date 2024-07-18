use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::{establish_connection, supplier};

#[derive(Serialize, Deserialize)]
pub struct SupplierListRequest {
    pub search_string: String,
}

pub async fn list_suppliers(payload: web::Json<SupplierListRequest>) -> impl Responder {
    let mut connection = establish_connection();
    let search: &String = &payload.search_string;
    match supplier::list_suppliers(&mut connection,&search) {
        Ok(suppliers) => HttpResponse::Ok().json(suppliers),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching suppliers"),
    }
}
