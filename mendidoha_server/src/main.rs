extern crate diesel;

mod db;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};

use env_logger::Env;
use handlers::supplier_handler::list_suppliers;
use handlers::user_handler::{greet, login, logout, reset_password, signup};
use mendidoha_server::middlewares::validation::ValidationMiddleware;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(ValidationMiddleware {
                exception_paths: vec![
                    "/signup".to_string(),
                    "/login".to_string(),
                    "/reset_password".to_string(),
                    "/logout".to_string(),
                ],
            })
            .route("/", web::get().to(greet))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
            .route("/logout", web::post().to(logout))
            .route("/suppliers", web::post().to(list_suppliers))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
