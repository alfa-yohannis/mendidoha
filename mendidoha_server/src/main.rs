use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use mendidoha_server::util::{get_session, set_session};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    // Generate a random key for cookie signing
    let private_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(), 
                private_key.clone()
            ) 
            )
            .route("/set_session", web::get().to(set_session))
            .route("/get_session", web::get().to(get_session))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


// extern crate diesel;

// mod db;
// mod handlers;
// mod schema;
// mod models;

// use actix_web::{web, App, HttpResponse, HttpServer};
// use actix_session::{Session, SessionMiddleware};
// use actix_web::cookie::Key;

// use handlers::user_handler::{greet, login, signup, reset_password, logout};
// use handlers::supplier_handler::list_suppliers;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let secret_key = Key::generate();

//         App::new()
//         .wrap(SessionMiddleware::new(
//                             actix_session::storage::CookieSessionStore::default(),
//                             secret_key.clone(),
//                         ))
//             .route("/", web::get().to(greet))
//             .route("/signup", web::post().to(signup))
//             .route("/login", web::post().to(login))
//             .route("/reset_password", web::post().to(reset_password))
//             .route("/logout", web::post().to(logout))
//             .route("/suppliers", web::post().to(list_suppliers)) 
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// // /// Handler function to get current session information
// // async fn get_session(session: Session) -> HttpResponse {
// //     // Example: Retrieve username from session
// //     if let Some(username) = session.get::<String>("username").unwrap_or(None) {
// //         HttpResponse::Ok().body(format!("Current active session for user: {}", username))
// //     } else {
// //         HttpResponse::NotFound().body("No active session found")
// //     }
// // }