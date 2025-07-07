use crate::module::user::dto::LoginDTO;
use crate::module::user::service;
use actix_web::{ResponseError, HttpResponse, Responder, post, web};
use validator::Validate;

use crate::config::common::DbPool;

#[post("/login")]
async fn login(pool: web::Data<DbPool>, body: web::Json<LoginDTO>) -> impl Responder {

    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    match service::login(&pool, body.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err.error_response(), // ✅ Don't end match block with `;`
    }
}
