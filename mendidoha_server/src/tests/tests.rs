use actix_web::{test, web, App};
use serde_json::json;
use mockito::mock;

use crate::handlers::{signup, login};
use crate::db::{create_user, verify_user, establish_connection, hash_password};
use crate::models::{SignUpRequest, LoginRequest};

// #[actix_rt::test]
// async fn test_signup_success() {
//     // Mock HTTP server
//     let _m = mock("POST", "/signup")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(r#"{"success": true, "message": "User signed up successfully"}"#)
//         .create();

//     // Mock database connection (using an in-memory database or similar mock)
//     let mut connection = establish_connection(); // Replace with mock if needed

//     // Prepare signup request payload
//     let signup_request = SignUpRequest {
//         username: "admin".to_string(),
//         password: "1234".to_string(),
//         first_name: "Test".to_string(),
//         middle_name: Some("Middle".to_string()),
//         last_name: "User".to_string(),
//     };

//     // Hash password
//     let hashed_password = hash_password(&signup_request.password);

//     // Create HTTP request
//     let req = test::TestRequest::post()
//         .uri("/signup")
//         .set_json(&signup_request)
//         .to_request();

//     // Call signup handler
//     let resp = signup(web::Json(signup_request), web::Data::new(connection.clone())).await;

//     // Assert response
//     assert_eq!(resp.status(), 200);

//     // Assert response body
//     let body = test::read_body(resp).await;
//     assert_eq!(body, r#"{"success":true,"message":"User signed up successfully"}"#);

//     // Optionally, assert database state or additional logic
// }

#[actix_rt::test]
async fn test_login_success() {
    // Mock HTTP server
    let _m = mock("GET", "/login?username=testuser&password=password")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"success": true, "message": "Login successful"}"#)
        .create();

    // Mock database connection (using an in-memory database or similar mock)
    let mut connection = establish_connection(); // Replace with mock if needed

    // Prepare login request query
    let login_request = LoginRequest {
        username: "testuser".to_string(),
        password: "password".to_string(),
    };

    // Create HTTP request
    let req = test::TestRequest::get()
        .uri("/login")
        .set_query(&login_request)
        .to_request();

    // Call login handler
    let resp = login(web::Query(login_request), web::Data::new(connection.clone())).await;

    // Assert response
    assert_eq!(resp.status(), 200);

    // Assert response body
    let body = test::read_body(resp).await;
    assert_eq!(body, r#"{"success":true,"message":"Login successful"}"#);

    // Optionally, assert additional logic or database interactions
}
