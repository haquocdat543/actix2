use crate::module::user::{
    dto::{InfoDTO, PutInfoDTO},
    service::UserService,
};
use crate::share::jwt::Claims;
use actix_web::HttpRequest;
use actix_web::{HttpMessage, HttpResponse, Responder, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

pub async fn put_info(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    body: web::Json<PutInfoDTO>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let username = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let body = body.into_inner();
    match UserService::update_info(db.get_ref(), username, InfoDTO::Put(body)).await {
        Ok(true) => HttpResponse::Ok().body("Info putted"),
        Ok(false) => HttpResponse::Ok().body("Put failed"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
