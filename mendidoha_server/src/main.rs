extern crate diesel;

mod db;
mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use env_logger::Env;

use handlers::good_handler::{add_goods, delete_goods, get_goods, list_goods, update_goods};
use handlers::supplier_handler::{
    add_supplier, delete_supplier, get_supplier, list_suppliers, update_supplier,
};
use handlers::user_handler::{delete_user, get_user, greet, login, logout, reset_password, signup};
use mendidoha_server::middlewares::validation::ValidationMiddleware;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
            // users
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
            .route("/logout", web::post().to(logout))
            .route("/delete_user", web::post().to(delete_user))
            .route("/get_user", web::post().to(get_user))
            // suppliers
            .route("/suppliers", web::post().to(list_suppliers))
            .route("/suppliers/add", web::post().to(add_supplier))
            .route("/suppliers/update", web::post().to(update_supplier))
            .route("/suppliers/delete", web::post().to(delete_supplier))
            .route("/suppliers/get", web::post().to(get_supplier))
            // goods
            .route("/goods", web::post().to(list_goods))
            .route("/goods/add", web::post().to(add_goods))
            .route("/goods/update", web::post().to(update_goods))
            .route("/goods/delete", web::post().to(delete_goods))
            .route("/goods/get", web::post().to(get_goods))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
