use actix_web::HttpMessage;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sea_orm::DatabaseConnection;

use crate::module::user::service::UserService;
use crate::share::jwt::Claims;

pub async fn get_user(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    // ✅ Safely get and clone username
    let username = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    match UserService::get_user(db.get_ref(), username).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
