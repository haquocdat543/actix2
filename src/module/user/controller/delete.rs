use crate::module::user::service::UserService;
use actix_web::{HttpResponse, Responder, delete, web};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[delete("/{id}")]
pub async fn delete(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    match UserService::remove_user(db.get_ref(), id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
