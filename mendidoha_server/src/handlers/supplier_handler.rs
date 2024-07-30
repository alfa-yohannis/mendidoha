use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::{establish_connection, supplier};

#[derive(Serialize, Deserialize)]
pub struct SupplierListRequest {
    pub search_string: String,
}

#[derive(Serialize, Deserialize)]
pub struct SupplierRequest {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SupplierUpdateRequest {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SupplierDeleteRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SupplierGetRequest {
    pub code: String,
}

pub async fn list_suppliers(payload: web::Json<SupplierListRequest>) -> impl Responder {
    let mut connection = establish_connection();

    let search: &String = &payload.search_string;
    match supplier::list_suppliers(&mut connection, &search) {
        Ok(suppliers) => HttpResponse::Ok().json(suppliers),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching suppliers"),
    }
}

pub async fn add_supplier(payload: web::Json<SupplierRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match supplier::create_supplier(&mut connection, &payload.code, &payload.name, None) {
        Ok(new_supplier) => HttpResponse::Ok().json(new_supplier),
        Err(_) => HttpResponse::InternalServerError().json("Error creating supplier"),
    }
}

pub async fn update_supplier(payload: web::Json<SupplierUpdateRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match supplier::update_supplier(
        &mut connection,
        &payload.code,
        &payload.name,
        None,
    ) {
        Ok(updated_supplier) => HttpResponse::Ok().json(updated_supplier),
        Err(_) => HttpResponse::InternalServerError().json("Error updating supplier"),
    }
}

pub async fn delete_supplier(payload: web::Json<SupplierDeleteRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match supplier::delete_supplier(&mut connection, &payload.code) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                HttpResponse::Ok().json("Supplier deleted")
            } else {
                HttpResponse::NotFound().json("Supplier not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Error deleting supplier"),
    }
}

pub async fn get_supplier(payload: web::Json<SupplierGetRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match supplier::get_supplier(&mut connection, &payload.code) {
        Ok(supplier) => HttpResponse::Ok().json(supplier),
        Err(_) => HttpResponse::NotFound().json("Supplier not found"),
    }
}
