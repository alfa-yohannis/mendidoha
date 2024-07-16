// use actix_web::{test, web, App, HttpResponse};
// use actix_session::{Session, SessionStatus};
// use mendidoha_server::handlers::supplier_handler::list_suppliers; // Adjust the path as needed
// use mendidoha_server::db::{establish_connection, supplier};

// #[actix_rt::test]
// async fn test_list_suppliers_authorized() {
//     // Simulate a logged-in session 
//     let mut app = test::init_service(
//         App::new()
//             .service(web::resource("/list_suppliers").route(web::get().to(list_suppliers))) // Adjust route setup as needed
//     ).await;

//     // Replace with actual session data setup
//     let mut session = Session::new(SessionStatus::Started);
//     session.set("username", "testuser".to_string()).unwrap();

//     let req = test::TestRequest::get()
//         .uri("/list_suppliers")
//         .to_request();

//     let response = test::call_service(&mut app, req.set_session(session)).await;
//     assert_eq!(response.status(), 200);

//     // You might want to validate the response body as well, depending on what `list_suppliers` returns.
// }

// #[actix_rt::test]
// async fn test_list_suppliers_unauthorized() {
//     let mut app = test::init_service(
//         App::new()
//             .data(AppState::new()) // Use App data for testing
//             .service(web::resource("/list_suppliers").route(web::get().to(list_suppliers))) // Adjust route setup as needed
//     ).await;

//     let req = test::TestRequest::get()
//         .uri("/list_suppliers")
//         .to_request();

//     let response = test::call_service(&mut app, req).await;
//     assert_eq!(response.status(), 401);
// }

// // Define your AppState struct if needed
// struct AppState {}

// impl AppState {
//     fn new() -> Self {
//         // Initialize your application state as needed
//         AppState {}
//     }
// }
