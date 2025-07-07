use crate::module::user::service::UserService;
use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[post("/seed")]
pub async fn seed(db: web::Data<DatabaseConnection>) -> impl Responder {
    match UserService::seed(db.get_ref()).await {
        Ok(()) => {
            let seeding_status = Response {
                message: "Seeded".to_string(),
            };
            HttpResponse::Created().json(seeding_status)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
