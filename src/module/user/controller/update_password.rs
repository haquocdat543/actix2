use crate::module::user::{dto::UpdatePasswordDTO, service::UserService};
use actix_web::{HttpResponse, Responder, patch, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[patch("/password")]
pub async fn update_password(
    db: web::Data<DatabaseConnection>,
    req: web::Json<UpdatePasswordDTO>,
) -> impl Responder {
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let req = req.into_inner();
    match UserService::update_password(db.get_ref(), req.name, req.password, req.new_password).await
    {
        Ok(true) => HttpResponse::Ok().body("Password updated"),
        Ok(false) => HttpResponse::Ok().body("Password incorrect"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
