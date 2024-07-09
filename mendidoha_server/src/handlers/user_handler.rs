use actix_web::{web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

use crate::db::{create_user, establish_connection, verify_user};

use crate::db::hash_password;

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

// Structure for the sign-up request
#[derive(Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

// Structure for the sign-up response
#[derive(Serialize)]
pub struct SignUpResponse {
    success: bool,
    message: String,
}

// Handler for the sign-up endpoint
pub async fn signup(payload: web::Json<SignUpRequest>) -> impl Responder {
    let mut connection = establish_connection();

    // Hash the password using MD5 (or other hash function)
    let hashed_password = hash_password(&payload.password);

    // Create a new user in the database
    let new_user = create_user(
        &mut connection,
        &payload.username,
        &hashed_password,
        &payload.first_name,
        payload.middle_name.as_deref(),
        &payload.last_name,
    );

    match new_user {
        Ok(_) => HttpResponse::Ok().json(SignUpResponse {
            success: true,
            message: "User signed up successfully".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(SignUpResponse {
            success: false,
            message: "Failed to sign up user".to_string(),
        }),
    }
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
