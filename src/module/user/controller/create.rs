use crate::module::user::dto::CreateUserDTO;
use crate::module::user::service;
use actix_web::{ResponseError, HttpResponse, Responder, post, web};
use validator::Validate;

use crate::config::common::DbPool;

#[post("")]
async fn create(pool: web::Data<DbPool>, body: web::Json<CreateUserDTO>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    match service::create_user(&pool, body.into_inner()) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => err.error_response(), // ✅ Don't end match block with `;`
    }
}
