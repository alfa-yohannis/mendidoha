use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::{establish_connection, goods};

#[derive(Serialize, Deserialize)]
pub struct GoodsListRequest {
    pub search_string: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsRequest {
    pub code: String,
    pub name: String,
    pub currency: String, // Moved currency before price
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsUpdateRequest {
    pub code: String,
    pub name: String,
    pub currency: String, // Moved currency before price
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsDeleteRequest {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsGetRequest {
    pub code: String,
}

pub async fn list_goods(payload: web::Json<GoodsListRequest>) -> impl Responder {
    let mut connection = establish_connection();

    let search: &String = &payload.search_string;
    match goods::list_goods(&mut connection, &search) {
        Ok(goods) => HttpResponse::Ok().json(goods),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching goods"),
    }
}

pub async fn add_goods(payload: web::Json<GoodsRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match goods::create_goods(
        &mut connection,
        &payload.code,
        &payload.name,
        &payload.currency, // Included currency before price
        payload.price,
        payload.quantity,
        payload.unit.as_deref(),
        None,
    ) {
        Ok(new_goods) => HttpResponse::Ok().json(new_goods),
        Err(_) => HttpResponse::InternalServerError().json("Error creating goods"),
    }
}

pub async fn update_goods(payload: web::Json<GoodsUpdateRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match goods::update_goods(
        &mut connection,
        &payload.code,
        &payload.name,
        &payload.currency, // Included currency before price
        payload.price,
        payload.quantity,
        payload.unit.as_deref(),
        None,
    ) {
        Ok(updated_goods) => HttpResponse::Ok().json(updated_goods),
        Err(_) => HttpResponse::InternalServerError().json("Error updating goods"),
    }
}

pub async fn delete_goods(payload: web::Json<GoodsDeleteRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match goods::delete_goods(&mut connection, &payload.code) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                HttpResponse::Ok().json("Goods deleted")
            } else {
                HttpResponse::NotFound().json("Goods not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Error deleting goods"),
    }
}

pub async fn get_goods(payload: web::Json<GoodsGetRequest>) -> impl Responder {
    let mut connection = establish_connection();

    match goods::get_goods(&mut connection, &payload.code) {
        Ok(goods) => HttpResponse::Ok().json(goods),
        Err(_) => HttpResponse::NotFound().json("Goods not found"),
    }
}
