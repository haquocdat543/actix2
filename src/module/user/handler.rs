use actix_web::web;

use super::controller::get_users::get_users;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user").service(get_users)
}
