use mendidoha_server::handlers::user_handler;
use mendidoha_server::handlers::user_handler::{greet, login, reset_password, signup}; // Adjust the path as needed
use mendidoha_server::handlers::user_handler::{LoginResponse, SignUpResponse};
// Adjust the path as needed

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, web, App};

    use mendidoha_server::handlers::user_handler::{
        logout, GetUserResponse, UpdatePasswordResponse,
    };
    use serde_json::json;

    #[actix_rt::test]
    async fn test_signup_success() {
        let mut app = test::init_service(App::new().route("/signup", web::post().to(signup))).await;

        // find existing if success delete
        let username = "success_user";
        let password = "succcess_user";
        let response = get_user(username, password).await;
        if response.success {
            delete_user(username, password).await;
        }

        // add a new one
        let payload = json!({
            "username": username,
            "password": password,
            "first_name": "Test",
            "middle_name": null,
            "last_name": "User"
        });

        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: SignUpResponse = test::read_body_json(resp).await;
        assert!(response_body.success);

        delete_user(username, password).await;
    }

    #[actix_rt::test]
    async fn test_signup_failure() {
        let mut app = test::init_service(App::new().route("/signup", web::post().to(signup))).await;

        // find existing if success delete
        let username = "fail_user";
        let password = "fail_user";
        let response = get_user(username, password).await;
        if !response.success {
            create_user(username, password).await;   
        }

        // Simulate a failure in user creation by providing invalid input or mocking the function
        let payload = json!({
            "username": username,
            "password": format!("{}A", password),
            "first_name": "Test",
            "middle_name": null,
            "last_name": "User"
        });
        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);

        let response_body: SignUpResponse = test::read_body_json(resp).await;
        assert!(!response_body.success);
        assert_eq!(response_body.message, "Failed to sign up user");

        delete_user(username, password).await;
    }

    #[actix_rt::test]
    async fn test_login_success() {
        let mut app = test::init_service(App::new().route("/login", web::post().to(login))).await;

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
    }

    #[actix_rt::test]
    async fn test_login_failure() {
        let mut app = test::init_service(App::new().route("/login", web::post().to(login))).await;

        let payload = json!({
            "username": "wronguser",
            "password": "wrongpassword",
            "device_id": "device123"
        });

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: LoginResponse = test::read_body_json(resp).await;
        assert!(!response_body.success);
    }

    #[actix_rt::test]
    async fn test_reset_password_success() {
        
        // find existing if success delete
        let username = "reset_password_user";
        let password = &username;
        let response = get_user(username, password).await;
        if !response.success {
            create_user(username, password).await;
        }
        
        let mut app =
            test::init_service(App::new().route("/reset_password", web::post().to(reset_password)))
                .await;

        let payload = json!({
            "username": username,
            "reset_code": "1234",
            "new_password":format!("{}A", password)
        });

        let req = test::TestRequest::post()
            .uri("/reset_password")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: UpdatePasswordResponse = test::read_body_json(resp).await;
        assert!(response_body.success);
        assert_eq!(response_body.message, "Password updated successfully");

        delete_user(username, password).await;
    }

    #[actix_rt::test]
    async fn test_reset_password_failure() {
        let mut app =
            test::init_service(App::new().route("/reset_password", web::post().to(reset_password)))
                .await;

        let payload = json!({
            "username": "wronguser",
            "reset_code": "wrongreset",
            "new_password": "newpassword123"
        });

        let req = test::TestRequest::post()
            .uri("/reset_password")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: UpdatePasswordResponse = test::read_body_json(resp).await;
        assert!(!response_body.success);
    }

    #[actix_rt::test]
    async fn test_greet() {
        let mut app = test::init_service(App::new().route("/greet", web::get().to(greet))).await;

        let req = test::TestRequest::get().uri("/greet").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, User!");
    }

    #[actix_rt::test]
    async fn test_logout() {
        let mut app = test::init_service(App::new().route("/logout", web::get().to(logout))).await;

        let req = test::TestRequest::get().uri("/logout").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Logged out successfully");
    }

    pub async fn create_user(username: & str, password: &str) {
        let mut app = test::init_service(App::new().route("/signup", web::post().to(signup))).await;

        // add a new one
        let payload = json!({
            "username": username,
            "password": password,
            "first_name": "Test",
            "middle_name": null,
            "last_name": "User"
        });

        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: SignUpResponse = test::read_body_json(resp).await;
        assert!(response_body.success);
    }

    pub async fn get_user(username: &str, password: &str) -> GetUserResponse {
        // delete the user
        let mut app = test::init_service(
            App::new().route("/get_user", web::post().to(user_handler::get_user)),
        )
        .await;

        let payload = json!({
            "username": username,
            "password": password,
        });

        let req: actix_http::Request = test::TestRequest::post()
            .uri("/get_user")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body: GetUserResponse = test::read_body_json(resp).await;
        return response_body;
    }

    pub async fn delete_user(username: &str, password: &str) {
        // delete the user
        let mut app = test::init_service(
            App::new().route("/delete_user", web::post().to(user_handler::delete_user)),
        )
        .await;

        let payload = json!({
            "username": username,
            "password": password,
        });

        let req: actix_http::Request = test::TestRequest::post()
            .uri("/delete_user")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
    }
}
