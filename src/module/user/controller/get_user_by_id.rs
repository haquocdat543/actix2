use crate::module::user::service::UserService;
use actix_web::{HttpResponse, Responder, get, web};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[get("/{id}")]
pub async fn get_user_by_id(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match UserService::get_user_by_id(db.get_ref(), id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
