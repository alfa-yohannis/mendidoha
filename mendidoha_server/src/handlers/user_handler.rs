use actix_web::{web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use crate::db::{establish_connection, verify_user};

// Structure for the login request
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Structure for the login response
#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
}

// Handler for the login endpoint using URL parameters
pub async fn login(query: web::Query<LoginRequest>) -> impl Responder {
    let mut connection = establish_connection();

    if verify_user(&mut connection, &query.username, &query.password) {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        HttpResponse::Ok().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
        })
    }
}

// Simple greeting endpoint
pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Microservice!")
}