extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};
use std::env;

mod db;
mod handlers;

use db::models::User;
use db::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    user_id: &'a str,
    username: &'a str,
    password: &'a str,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_user_id() -> String {
    let mut rng = thread_rng();
    let range = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&range).to_string()).collect()
}

pub fn create_user<'a>(conn: &mut PgConnection, username: &'a str, password: &'a str) -> User {
    let random_user_id = generate_user_id();

    let new_user = NewUser {
        user_id: &random_user_id,
        username,
        password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

// Structure for the login request
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Structure for the login response
#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

// Function to verify username and password
pub fn verify_user(conn: &mut PgConnection, _username: &str, _password: &str) -> bool {
    use db::schema::users::dsl::*;

    match users
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .first::<User>(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Handler for the login endpoint using URL parameters
async fn login(query: web::Query<LoginRequest>) -> impl Responder {
    let mut connection = establish_connection();
    
    if verify_user(&mut connection, &query.username, &query.password) {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
        })
    }
}

// Simple greeting endpoint
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Microservice!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/login", web::get().to(login))  // Change to GET and handle URL parameters
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}