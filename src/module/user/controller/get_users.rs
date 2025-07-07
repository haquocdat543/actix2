use crate::module::user::service;
use actix_web::{HttpResponse, Responder, get, web};

use crate::config::common::DbPool;

#[get("/all")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let user = service::get_users(&pool);
    HttpResponse::Ok().json(user)
}
