use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Cookie;
use actix_web::cookie::Key;
use actix_web::dev::Service;
use actix_web::http::StatusCode;
use actix_web::{test, App};
use mendidoha_server::util::{get_session, set_session}; // Adjust import path as needed

#[actix_rt::test]
async fn test_session_management() {
    let private_key = Key::generate();

    let app = test::init_service(
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), private_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .route("/set_session", actix_web::web::get().to(set_session))
            .route("/get_session", actix_web::web::get().to(get_session)),
    )
    .await;

    // Test setting the session
    let req = test::TestRequest::get().uri("/set_session").to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Extract the session cookie
    // let mut session_cookie: Option<Cookie<'_>> = None;
    // for cookie in resp.response().cookies() {
    //     let x = cookie.name();
    //     println!("Test: {:?}", cookie.name());
    //     if cookie.name() == "id" {
    //         session_cookie = Some(cookie.clone());
    //         break;
    //     }
    // }
    let session_cookie: Option<Cookie> = resp.response().cookies().next();

    // Assert that the session cookie was found
    let session_cookie = session_cookie.expect("Session cookie not found");

    // Test getting the session with the session cookie
    let req = test::TestRequest::get()
        .uri("/get_session")
        .cookie(session_cookie.clone())
        .to_request();
    let resp = app.call(req).await.unwrap();

    // Check if response status is OK
    assert_eq!(resp.status(), StatusCode::OK);

    // Read the response body
    let body = test::read_body(resp).await;

    let body_str = String::from_utf8(body.to_vec()).expect("Response body is not valid UTF-8");

    // Ensure body contains expected content
    assert_eq!(body_str, "User ID: 42");
}
