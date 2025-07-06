use crate::module::user::handler;
use actix_web::web;

pub fn api_scope() -> actix_web::Scope {
    web::scope("/api").service(handler::user_scope())
}
