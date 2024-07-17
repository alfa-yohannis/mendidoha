use crate::db::sessions::{create_session, get_active_session};
use crate::db::user::{
    create_user, get_user_code_by_username, update_user_password, verify_user, verify_user_by_code,
};
use crate::db::{establish_connection, hash_password};
use crate::models::session::NewSession;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Duration, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub session_id: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub expiry_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub username: String,
    pub reset_code: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePasswordResponse {
    pub success: bool,
    pub message: String,
}

pub async fn signup(payload: web::Json<SignUpRequest>) -> impl Responder {
    let mut connection = establish_connection();

    let hashed_password = hash_password(&payload.password);

    let new_user = create_user(
        &mut connection,
        &payload.username,
        &hashed_password,
        &payload.first_name,
        payload.middle_name.as_deref(),
        &payload.last_name,
        Some(&payload.username),
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

pub async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
    let mut connection = establish_connection();

    if verify_user(&mut connection, &payload.username, &payload.password) {
        // Fetch user code by username
        let user_code = get_user_code_by_username(&mut connection, &payload.username);

        if let Some(user_code) = user_code {
            // Check if there's an active session for this user and device
            match get_active_session(&mut connection, &user_code, &payload.device_id) {
                Ok(Some(session)) => {
                    // Active session exists, return it
                    HttpResponse::Ok().json(LoginResponse {
                        success: true,
                        message: "Active session found".to_string(),
                        session_id: Some(session.session_id),
                        start_time: Some(session.start_time),
                        expiry_time: Some(session.expiry_time),
                    })
                }
                Ok(None) => {
                    // No active session, create a new one
                    let session_id = generate_session_id();

                    let start_time = Utc::now();
                    let expiry_time = Utc::now() + Duration::hours(48);

                    let new_session = NewSession {
                        user_code: &user_code,
                        device_id: &payload.device_id,
                        session_id: &session_id,
                        start_time,
                        expiry_time,
                        created: Utc::now(),
                        updated: Utc::now(),
                        created_by: Some(&payload.username),
                        updated_by: Some(&payload.username),
                    };

                    if let Err(_) = create_session(&mut connection, &new_session) {
                        return HttpResponse::InternalServerError().json(LoginResponse {
                            success: false,
                            message: "Failed to create session".to_string(),
                            session_id: None,
                            start_time: None,
                            expiry_time: None,
                        });
                    }

                    HttpResponse::Ok().json(LoginResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        session_id: Some(session_id),
                        start_time: Some(start_time),
                        expiry_time: Some(expiry_time),
                    })
                }
                Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
                    success: false,
                    message: "Failed to check active session".to_string(),
                    session_id: None,
                    start_time: None,
                    expiry_time: None,
                }),
            }
        } else {
            HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                message: "Failed to retrieve user information".to_string(),
                session_id: None,
                start_time: None,
                expiry_time: None,
            })
        }
    } else {
        HttpResponse::Ok().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            session_id: None,
            start_time: None,
            expiry_time: None,
        })
    }
}

fn generate_session_id() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

pub async fn reset_password(payload: web::Json<UpdatePasswordRequest>) -> impl Responder {
    let mut connection = establish_connection();

    if verify_user_by_code(&mut connection, &payload.username, &payload.reset_code) {
        let hashed_new_password = hash_password(&payload.new_password);

        let update_result =
            update_user_password(&mut connection, &payload.username, &hashed_new_password);

        match update_result {
            Ok(_) => HttpResponse::Ok().json(UpdatePasswordResponse {
                success: true,
                message: "Password updated successfully".to_string(),
            }),
            Err(_) => HttpResponse::InternalServerError().json(UpdatePasswordResponse {
                success: false,
                message: "Failed to update password".to_string(),
            }),
        }
    } else {
        HttpResponse::Ok().json(UpdatePasswordResponse {
            success: false,
            message: "Invalid username or old password".to_string(),
        })
    }
}

pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, User!"))
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logged out successfully")
}
