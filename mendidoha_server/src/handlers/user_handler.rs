use actix_session::Session;
use actix_web::{web,  HttpResponse,  Responder};
use serde_derive::{Deserialize, Serialize};
use crate::db::{create_user, establish_connection, update_user_password, verify_user, verify_user_by_code};
use crate::db::hash_password;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
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

pub async fn login(payload: web::Json<LoginRequest>, session: Session) -> impl Responder {
    let mut connection = establish_connection();

    if verify_user(&mut connection, &payload.username, &payload.password) {
        session.insert("username", &payload.username).unwrap();

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

pub async fn reset_password(payload: web::Json<UpdatePasswordRequest>, session: Session) -> impl Responder {
    let mut connection = establish_connection();

    if let Some(username) = session.get::<String>("username").unwrap() {
        if verify_user_by_code(&mut connection, &username, &payload.reset_code) {
            let hashed_new_password = hash_password(&payload.new_password);

            let update_result = update_user_password(&mut connection, &username, &hashed_new_password);

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
    } else {
        HttpResponse::Unauthorized().json(UpdatePasswordResponse {
            success: false,
            message: "Unauthorized access".to_string(),
        })
    }
}

pub async fn greet(session: Session) -> impl Responder {
    if let Some(username) = session.get::<String>("username").unwrap() {
        HttpResponse::Ok().body(format!("Hello, {}!", username))
    } else {
        HttpResponse::Unauthorized().body("Unauthorized access")
    }
}

pub async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Ok().body("Logged out successfully")
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let secret_key = Key::generate();

//         App::new()
//             .wrap(SessionMiddleware::new(
//                 actix_session::storage::CookieSessionStore::default(),
//                 secret_key.clone(),
//             ))
//             .service(web::scope("/api")
//                 .route("/signup", web::post().to(signup))
//                 .route("/login", web::post().to(login))
//                 .route("/reset-password", web::post().to(reset_password))
//                 .route("/greet", web::get().to(greet))
//                 .route("/logout", web::post().to(logout))
//             )
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
