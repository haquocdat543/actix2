use actix_web::web;
use crate::module::user::controller;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(controller::create::create)
        .service(controller::get_users::get_users)
}
