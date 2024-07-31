mod good_handler_tests {

    use actix_web::{http, test, web, App};
    use serde_json::json;

    use mendidoha_server::{
        handlers::good_handler::{add_goods, delete_goods, get_goods, list_goods, update_goods},
        models::good::Goods,
    };

    #[actix_rt::test]
    async fn test_list_goods() {
        let mut app =
            test::init_service(App::new().route("/goods", web::post().to(list_goods))).await;

        // Request goods list with a specific search string
        let list_payload = json!({
            "search_string": "",
        });

        let list_req = test::TestRequest::post()
            .uri("/goods")
            .set_json(&list_payload)
            .to_request();

        let list_resp = test::call_service(&mut app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);

        let response_body: Vec<Goods> = test::read_body_json(list_resp).await;
        assert!(!response_body.is_empty()); // Adjust based on your test data
    }

    #[actix_rt::test]
    async fn test_add_goods() {
        let mut app = test::init_service(
            App::new()
                .route("/goods/add", web::post().to(add_goods))
                .route("/goods/delete", web::post().to(delete_goods)),
        )
        .await;

        let add_payload = json!({
            "code": "1000000006",
            "name": "New Goods",
            "currency": "USD",
            "price": 123.45,
            "quantity": 10,
            "unit": "units",
        });

        let add_req = test::TestRequest::post()
            .uri("/goods/add")
            .set_json(&add_payload)
            .to_request();

        let add_resp = test::call_service(&mut app, add_req).await;
        assert_eq!(add_resp.status(), http::StatusCode::OK);

        let response_body: Goods = test::read_body_json(add_resp).await;
        assert_eq!(response_body.name, "New Goods");

        // Clean up by deleting the dummy goods
        let delete_payload = json!({
            "code": response_body.code,
        });

        let delete_req = test::TestRequest::post()
            .uri("/goods/delete")
            .set_json(&delete_payload)
            .to_request();

        let delete_resp = test::call_service(&mut app, delete_req).await;
        assert_eq!(delete_resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_goods() {
        let mut app = test::init_service(
            App::new()
                .route("/goods/add", web::post().to(add_goods))
                .route("/goods/update", web::post().to(update_goods))
                .route("/goods/delete", web::post().to(delete_goods)),
        )
        .await;

        // Add a dummy goods entry
        let add_payload = json!({
            "code": "1000000007",
            "name": "Old Goods",
            "currency": "USD",
            "price": 100.00,
            "quantity": 20,
            "unit": "units",
        });

        let add_req = test::TestRequest::post()
            .uri("/goods/add")
            .set_json(&add_payload)
            .to_request();

        let add_resp = test::call_service(&mut app, add_req).await;
        assert_eq!(add_resp.status(), http::StatusCode::OK);

        let old_response_body: Goods = test::read_body_json(add_resp).await;

        // Update the dummy goods entry
        let update_payload = json!({
            "code": old_response_body.code,
            "name": "Updated Goods",
            "currency": "USD",
            "price": 150.00,
            "quantity": 25,
            "unit": "boxes",
        });

        let update_req = test::TestRequest::post()
            .uri("/goods/update")
            .set_json(&update_payload)
            .to_request();

        let update_resp = test::call_service(&mut app, update_req).await;
        assert_eq!(update_resp.status(), http::StatusCode::OK);

        let response_body: Goods = test::read_body_json(update_resp).await;
        assert_eq!(response_body.code, old_response_body.code);
        assert_eq!(response_body.name, "Updated Goods");

        // Clean up by deleting the dummy goods
        let delete_payload = json!({
            "code": old_response_body.code,
        });

        let delete_req = test::TestRequest::post()
            .uri("/goods/delete")
            .set_json(&delete_payload)
            .to_request();

        let delete_resp = test::call_service(&mut app, delete_req).await;
        assert_eq!(delete_resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_delete_goods() {
        let mut app = test::init_service(
            App::new()
                .route("/goods/add", web::post().to(add_goods))
                .route("/goods/delete", web::post().to(delete_goods)),
        )
        .await;

        // Add a dummy goods entry
        let add_payload = json!({
            "code": "1000000008",
            "name": "Goods to Delete",
            "currency": "USD",
            "price": 200.00,
            "quantity": 30,
            "unit": "units",
        });

        let add_req = test::TestRequest::post()
            .uri("/goods/add")
            .set_json(&add_payload)
            .to_request();

        let add_resp = test::call_service(&mut app, add_req).await;
        assert_eq!(add_resp.status(), http::StatusCode::OK);

        let old_response_body: Goods = test::read_body_json(add_resp).await;

        // Delete the dummy goods entry
        let delete_payload = json!({
            "code": old_response_body.code,
        });

        let delete_req = test::TestRequest::post()
            .uri("/goods/delete")
            .set_json(&delete_payload)
            .to_request();

        let delete_resp = test::call_service(&mut app, delete_req).await;
        assert_eq!(delete_resp.status(), http::StatusCode::OK);

        let response_body: String = test::read_body_json(delete_resp).await;
        assert_eq!(response_body, "Goods deleted");
    }

    #[actix_rt::test]
    async fn test_get_goods() {
        let mut app =
            test::init_service(App::new().route("/goods/get", web::post().to(get_goods))).await;

        // Get the goods
        let get_payload = json!({
            "code": "0000000120",
        });

        let get_req = test::TestRequest::post()
            .uri("/goods/get")
            .set_json(&get_payload)
            .to_request();

        let get_resp = test::call_service(&mut app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let response_body: Goods = test::read_body_json(get_resp).await;
        assert_eq!(response_body.code, "0000000120");
        assert_eq!(response_body.name, "Travel Pillow"); // Adjust based on your test data
    }
}
