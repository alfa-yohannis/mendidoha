extern crate diesel;

use actix_web::{web, App, HttpServer};
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;


mod db;
mod handlers;

use handlers::user_handler::{greet, login, signup, reset_password, logout};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let secret_key = Key::generate();

        App::new()
        .wrap(SessionMiddleware::new(
                            actix_session::storage::CookieSessionStore::default(),
                            secret_key.clone(),
                        ))
            .route("/", web::get().to(greet))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
            .route("/logout", web::post().to(logout))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
