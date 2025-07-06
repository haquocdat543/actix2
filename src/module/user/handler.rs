use actix_web::web;

use super::controller;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(controller::create::create)
        .service(controller::get_users::get_users)
        .service(controller::get_user_by_id::get_user_by_id)
        .service(controller::delete::delete)
}
