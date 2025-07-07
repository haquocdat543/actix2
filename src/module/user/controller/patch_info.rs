use crate::module::user::{dto::PatchInfoDTO, service::UserService};
use crate::share::jwt::Claims;
use actix_web::HttpRequest;
use actix_web::{HttpMessage, HttpResponse, Responder, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

pub async fn patch_info(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    body: web::Json<PatchInfoDTO>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let username = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let body = body.into_inner();
    match UserService::update_info(db.get_ref(), username, body).await {
        Ok(true) => HttpResponse::Ok().body("Info patched"),
        Ok(false) => HttpResponse::Ok().body("Patch failed"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
