use actix_session::Session;
use actix_web::{HttpResponse, Responder};

pub async fn set_session(session: Session) -> impl Responder {
    session.insert("user_id", 42).unwrap();
    let user_id = session.get::<i32>("user_id").unwrap();
    println!("Hasil SET: {:?}", user_id);
    HttpResponse::Ok().body("Session set")
}

pub async fn get_session(session: Session) -> impl Responder {
    let user_id = session.get::<i32>("user_id").unwrap();
    println!("Hasil GET: {:?}", user_id);
    if let Some(user_id) = session.get::<i32>("user_id").unwrap() {
        HttpResponse::Ok().body(format!("User ID: {}", user_id))
    } else {
        HttpResponse::Ok().body("No user ID found")
    }
}