use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::db::{establish_connection, supplier};

#[derive(Serialize, Deserialize)]
pub struct SupplierListRequest {
    pub search_string: String
}

pub async fn list_suppliers(payload: web::Json<SupplierListRequest>, session: Session) -> impl Responder {
    if let Some(_username) = session.get::<String>("username").unwrap() {
        let mut connection = establish_connection();
        let search: &String = &payload.search_string;
        match supplier::list_suppliers(&mut connection, Some(search.to_string())) {
            Ok(suppliers) => HttpResponse::Ok().json(suppliers),
            Err(_) => HttpResponse::InternalServerError().json("Error fetching suppliers"),
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized access")
    }
}
