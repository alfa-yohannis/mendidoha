use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};

use crate::db::{establish_connection, sessions::is_session_valid, supplier};

#[derive(Serialize, Deserialize)]
pub struct SupplierListRequest {
    pub search_string: String,
    pub session_id: String
}

pub async fn list_suppliers(payload: web::Json<SupplierListRequest>) -> impl Responder {

    let mut connection = establish_connection();

    let session_id = &payload.session_id;
    info!("session_id: {}", session_id);
    if !is_session_valid(&mut connection, session_id).unwrap_or(false) {
        return HttpResponse::Unauthorized().json("Invalid session ID");
    }

    let search: &String = &payload.search_string;
    match supplier::list_suppliers(&mut connection,&search) {
        Ok(suppliers) => HttpResponse::Ok().json(suppliers),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching suppliers"),
    }
}
