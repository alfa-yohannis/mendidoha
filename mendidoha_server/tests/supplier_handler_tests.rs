use actix_web::{http, test, web, App};

use mendidoha_server::{
    handlers::supplier_handler::{
        add_supplier, delete_supplier, get_supplier, list_suppliers, update_supplier,
    },
    models::supplier::Supplier,
};
use serde_json::json;

#[actix_rt::test]
async fn test_list_suppliers() {
    let mut app =
        test::init_service(App::new().route("/list_suppliers", web::post().to(list_suppliers)))
            .await;

    // Request supplier list with a specific search string
    let list_payload = json!({
        "search_string": "1000000003",
    });

    let list_req = test::TestRequest::post()
        .uri("/list_suppliers")
        .set_json(&list_payload)
        .to_request();

    let list_resp = test::call_service(&mut app, list_req).await;
    assert_eq!(list_resp.status(), http::StatusCode::OK);

    let response_body: Vec<Supplier> = test::read_body_json(list_resp).await;
    assert!(!response_body.is_empty());
    assert_eq!(response_body.len(), 1);
    assert_eq!(response_body[0].name, "Starlight Services LLC");
}

#[actix_rt::test]
async fn test_list_suppliers_empty_search_string() {
    let mut app =
        test::init_service(App::new().route("/list_suppliers", web::post().to(list_suppliers)))
            .await;

    // Request supplier list with an empty search string
    let list_payload = json!({
        "search_string": "",
    });

    let list_req = test::TestRequest::post()
        .uri("/list_suppliers")
        .set_json(&list_payload)
        .to_request();

    let list_resp = test::call_service(&mut app, list_req).await;
    assert_eq!(list_resp.status(), http::StatusCode::OK);

    let response_body: Vec<Supplier> = test::read_body_json(list_resp).await;
    assert!(!response_body.is_empty());
}

#[actix_rt::test]
async fn test_add_supplier() {
    let mut app = test::init_service(
        App::new()
            .route("/add_supplier", web::post().to(add_supplier))
            .route("/delete_supplier", web::post().to(delete_supplier)),
    )
    .await;

    let add_payload = json!({
        "code": "9000000003",
        "name": "New Supplier",
    });

    let add_req = test::TestRequest::post()
        .uri("/add_supplier")
        .set_json(&add_payload)
        .to_request();

    let add_resp = test::call_service(&mut app, add_req).await;
    assert_eq!(add_resp.status(), http::StatusCode::OK);

    let response_body: Supplier = test::read_body_json(add_resp).await;
    assert_eq!(response_body.name, "New Supplier");

    // Clean up by deleting the dummy supplier
    let delete_payload = json!({
        "code": response_body.code,
    });

    let delete_req = test::TestRequest::post()
        .uri("/delete_supplier")
        .set_json(&delete_payload)
        .to_request();

    let delete_resp = test::call_service(&mut app, delete_req).await;
    assert_eq!(delete_resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_update_supplier() {
    let mut app = test::init_service(
        App::new()
            .route("/add_supplier", web::post().to(add_supplier))
            .route("/update_supplier", web::post().to(update_supplier))
            .route("/delete_supplier", web::post().to(delete_supplier)),
    )
    .await;

    // Add a dummy supplier
    let add_payload = json!({
        "code": "9000000004",
        "name": "Old Supplier",
    });

    let add_req = test::TestRequest::post()
        .uri("/add_supplier")
        .set_json(&add_payload)
        .to_request();

    let add_resp = test::call_service(&mut app, add_req).await;
    assert_eq!(add_resp.status(), http::StatusCode::OK);

    let old_response_body: Supplier = test::read_body_json(add_resp).await;

    // Update the dummy supplier
    let update_payload = json!({
        "code": old_response_body.code,
        "name": "Updated Supplier",
    });

    let update_req = test::TestRequest::post()
        .uri("/update_supplier")
        .set_json(&update_payload)
        .to_request();

    let update_resp = test::call_service(&mut app, update_req).await;
    assert_eq!(update_resp.status(), http::StatusCode::OK);

    let response_body: Supplier = test::read_body_json(update_resp).await;
    assert_eq!(response_body.code, old_response_body.code);
    assert_eq!(response_body.name, "Updated Supplier");

    // Clean up by deleting the dummy supplier
    let delete_payload = json!({
        "code": old_response_body.code,
    });

    let delete_req = test::TestRequest::post()
        .uri("/delete_supplier")
        .set_json(&delete_payload)
        .to_request();

    let delete_resp = test::call_service(&mut app, delete_req).await;
    assert_eq!(delete_resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_delete_supplier() {
    let mut app = test::init_service(
        App::new()
            .route("/add_supplier", web::post().to(add_supplier))
            .route("/delete_supplier", web::post().to(delete_supplier)),
    )
    .await;

    // Add a dummy supplier
    let add_payload = json!({
        "code": "9000000005",
        "name": "Supplier to Delete",
    });

    let add_req = test::TestRequest::post()
        .uri("/add_supplier")
        .set_json(&add_payload)
        .to_request();

    let add_resp = test::call_service(&mut app, add_req).await;
    assert_eq!(add_resp.status(), http::StatusCode::OK);

    let old_response_body: Supplier = test::read_body_json(add_resp).await;

    // Delete the dummy supplier
    let delete_payload = json!({
        "code": old_response_body.code,
    });

    let delete_req = test::TestRequest::post()
        .uri("/delete_supplier")
        .set_json(&delete_payload)
        .to_request();

    let delete_resp = test::call_service(&mut app, delete_req).await;
    assert_eq!(delete_resp.status(), http::StatusCode::OK);

    let response_body: String = test::read_body_json(delete_resp).await;
    assert_eq!(response_body, "Supplier deleted");
}

#[actix_rt::test]
async fn test_get_supplier() {
    let mut app =
        test::init_service(App::new().route("/get_supplier", web::post().to(get_supplier))).await;

    // Get the supplier
    let get_payload = json!({
        "code": "1000000002",
    });

    let get_req = test::TestRequest::post()
        .uri("/get_supplier")
        .set_json(&get_payload)
        .to_request();

    let get_resp = test::call_service(&mut app, get_req).await;
    assert_eq!(get_resp.status(), http::StatusCode::OK);

    let response_body: Supplier = test::read_body_json(get_resp).await;
    assert_eq!(response_body.code, "1000000002");
    assert_eq!(response_body.name, "Global Solutions Inc.");
}
