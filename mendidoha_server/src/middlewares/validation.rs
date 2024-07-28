use actix_http::h1::Payload;
use actix_service::forward_ready;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::{self, BytesMut},
    App, Error, HttpMessage, HttpServer, Responder,
};
use futures_util::{future::LocalBoxFuture, stream::StreamExt};
use log::info;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RequestData {
    session_id: Option<String>,
    device_id: Option<String>,
    username: Option<String>,
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct ValidationMiddleware {
    pub exception_paths: Vec<String>,
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for ValidationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidationMiddlewareService {
            service: Rc::new(service),
            exception_paths: self.exception_paths.clone(),
        }))
    }
}

pub struct ValidationMiddlewareService<S> {
    service: Rc<S>,
    exception_paths: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for ValidationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let svc = self.service.clone();
        let exception_paths = self.exception_paths.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();

            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            let obj_result = serde_json::from_slice::<RequestData>(&body);
            if let Ok(obj) = obj_result {
                info!("{:?}", &obj);
                info!("{:?}", &obj.session_id);
                info!("{:?}", &obj.username);
                info!("{:?}", &obj.device_id);

                let paths_require_username = ["/login", "/signup"];
                let paths_require_device_id_only = ["/logout"];

                // Check if the device_id is missing
                if obj.device_id.is_none() {
                    return Err(actix_web::error::ErrorBadRequest("Missing device_id"));
                }

                // Allow the request to pass through if the path is in the exception list 
                // and the required fields are present
                if paths_require_username.contains(&req.path()) {
                    if obj.username.is_some() {
                        return call_service_with_payload(svc, req, body).await;
                    }
                } else if paths_require_device_id_only.contains(&req.path()) {
                    return call_service_with_payload(svc, req, body).await;
                } else if obj.session_id.is_some() && obj.username.is_some() {
                    return call_service_with_payload(svc, req, body).await;
                }
            }

            // If the fields are missing or deserialization failed, return an error
            Err(actix_web::error::ErrorBadRequest(
                "Missing session_id, username, or device_id",
            ))
        })
    }
}

async fn call_service_with_payload<S, B>(
    svc: Rc<S>,
    mut req: ServiceRequest,
    body: BytesMut,
) -> Result<ServiceResponse<B>, Error>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    let (_, mut payload) = Payload::create(true);
    payload.unread_data(body.into());
    req.set_payload(payload.into());

    let res = svc.call(req).await?;
    Ok(res)
}
