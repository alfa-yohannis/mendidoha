extern crate diesel;

use actix_web::{web, App, HttpServer};

mod db;
mod handlers;

use handlers::user_handler::{greet, login};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/login", web::get().to(login)) // Change to GET and handle URL parameters
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
