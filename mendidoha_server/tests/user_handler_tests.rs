use actix_web::{test, web, App};
use mendidoha_server::handlers::user_handler::{signup, login, reset_password, greet}; // Adjust the path as needed
use mendidoha_server::handlers::user_handler::{SignUpRequest, SignUpResponse, LoginRequest, LoginResponse, UpdatePasswordRequest, UpdatePasswordResponse}; // Adjust the path as needed

#[actix_rt::test]
async fn test_signup_success() {
    let username = "testuser";
    let password = "testpassword";
    let first_name = "Test";
    let last_name = "User";

    // Check if user already exists (pseudo code, replace with actual check)
    if !user_exists(username) {
        let payload = SignUpRequest {
            username: username.to_string(),
            password: password.to_string(),
            first_name: first_name.to_string(),
            middle_name: None,
            last_name: last_name.to_string(),
        };

        let mut app = test::init_service(App::new().route("/signup", web::post().to(signup))).await;

        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&payload)
            .to_request();

        let resp: SignUpResponse = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.success, true);
        assert_eq!(resp.message, "User signed up successfully");
    } else {
        println!("User already exists, skipping signup test");
    }
}

#[actix_rt::test]
async fn test_login_success() {
    let payload = LoginRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };

    let mut app = test::init_service(App::new().route("/login", web::post().to(login))).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(payload)
        .to_request();

    let resp: LoginResponse = test::call_and_read_body_json(&mut app, req).await;

    assert_eq!(resp.success, true);
    assert_eq!(resp.message, "Login successful");
}

#[actix_rt::test]
async fn test_reset_password_success() {
    let payload = UpdatePasswordRequest {
        username: "testuser".to_string(),
        reset_code: "1234".to_string(),
        new_password: "testpassword".to_string(),
    };

    let mut app = test::init_service(App::new().route("/reset_password", web::post().to(reset_password))).await;

    let req = test::TestRequest::post()
        .uri("/reset_password")
        .set_json(&payload)
        .to_request();

    let resp: UpdatePasswordResponse = test::call_and_read_body_json(&mut app, req).await;

    assert_eq!(resp.success, true);
    assert_eq!(resp.message, "Password updated successfully");
}

#[actix_rt::test]
async fn test_greet() {
    let mut app = test::init_service(App::new().route("/greet", web::get().to(greet))).await;

    let req = test::TestRequest::get().uri("/greet").to_request();

    let resp = test::call_and_read_body(&mut app, req).await;

    assert_eq!(resp, "Hello, Microservice!");
}

fn user_exists(username: &str) -> bool {
    // Implement your logic to check if user exists in the database
    // Return true if user exists, false otherwise
    // This is just a placeholder function, replace it with actual logic
    match username {
        "testuser" => true,
        _ => false,
    }
}
