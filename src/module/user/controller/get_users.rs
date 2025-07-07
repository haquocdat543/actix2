use crate::module::user::service::UserService;
use actix_web::{HttpResponse, Responder, web};
use sea_orm::DatabaseConnection;

pub async fn get_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    match UserService::get_users(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
