use actix_web::web;

use super::controller::get_users;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(get_users::get_all_users)
        .service(get_users::get_user_by_id)
        .service(get_users::register_user)
        .service(get_users::delete_user)
}
