use crate::module::user::dto::CreateUserDTO;
use crate::module::user::service::create_user;
use actix_web::{HttpResponse, Responder, post, web};
use validator::Validate;

use crate::config::common::DbPool;

#[post("")]
async fn create(pool: web::Data<DbPool>, body: web::Json<CreateUserDTO>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let user = create_user(&pool, body.into_inner());
    HttpResponse::Ok().json(user)
}
