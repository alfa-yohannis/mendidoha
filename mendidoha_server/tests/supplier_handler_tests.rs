use actix_web::{http, test, web, App};
use mendidoha_server::{
    handlers::{supplier_handler::list_suppliers, user_handler::{login, LoginResponse}},
    models::supplier::Supplier,
};
use serde_json::json;

#[actix_rt::test]
async fn test_list_suppliers_valid_session() {
    let mut app = test::init_service(
        App::new()
            .route("/list_suppliers", web::post().to(list_suppliers))
            .route("/login", web::post().to(login)),
        // Mock establish_connection and is_session_valid here
    )
    .await;

    // login first
    let payload = json!({
        "username": "admin",
        "password": "1234",
        "device_id": "fcec045a-84ab-4aec-8544-1a6566594955"
    });

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body: LoginResponse = test::read_body_json(resp).await;
    assert!(response_body.success);


    // request supplier list
    let payload = json!({
        "search_string": "Products",
        "session_id": response_body.session_id,
    });

    let req = test::TestRequest::post()
        .uri("/list_suppliers")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);

    let response_body: Vec<Supplier> = test::read_body_json(resp).await;
    assert!(!response_body.is_empty());
    assert_eq!(response_body.len(), 1); // Expecting exactly one supplier
    assert_eq!(response_body[0].name, "Oceanic Products Ltd."); // Verify the content
}

#[actix_rt::test]
async fn test_list_suppliers_invalid_session() {
    let mut app = test::init_service(
        App::new().route("/list_suppliers", web::post().to(list_suppliers)), // Mock establish_connection and is_session_valid here
    )
    .await;

    let payload = json!({
        "search_string": "",
        "session_id": "invalid_session",
    });

    let req = test::TestRequest::post()
        .uri("/list_suppliers")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);

    let response_body: String = test::read_body_json(resp).await;
    assert_eq!(response_body, "Invalid session ID");
}