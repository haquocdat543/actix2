use crate::share::auth_middleware::JwtMiddleware;
use actix_web::web;

use super::controller;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(controller::create::create)
        .service(controller::login::login)
        .service(controller::delete::delete)
        .service(controller::update_password::update_password)
        .route(
            "/all",
            web::get()
                .to(controller::get_users::get_users)
                .wrap(JwtMiddleware),
        )
        .service(controller::delete::delete)
}
