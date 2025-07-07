use crate::module::user::service;
use actix_web::{HttpResponse, Responder, ResponseError, get, web};

use crate::config::common::DbPool;

#[get("/all")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    match service::get_users(&pool) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => err.error_response(), // ✅ Again, no semicolon
    }
}
