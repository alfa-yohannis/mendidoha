extern crate diesel;

use actix_web::{web, App, HttpServer};

mod db;
mod handlers;

use handlers::user_handler::{greet, login, signup, reset_password};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
