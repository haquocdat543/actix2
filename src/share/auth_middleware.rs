use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::InternalError,
    http::StatusCode,
};
use futures::future::{LocalBoxFuture, Ready, ok};
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::share::jwt::verify_token;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization")
                && let Ok(auth_str) = auth_header.to_str()
                && auth_str.starts_with("Bearer ")
            {
                let token = auth_str.trim_start_matches("Bearer ").trim();

                match verify_token(token) {
                    Ok(token_data) => {
                        // Optionally store claims in request extensions
                        req.extensions_mut().insert(token_data.claims);
                        return service.call(req).await;
                    }
                    Err(_) => {
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                    }
                }
            }

            let json_body = json!({ "error": "Authorization header missing or malformed" });
            Err(InternalError::new(json_body, StatusCode::UNAUTHORIZED).into())
        })
    }
}
