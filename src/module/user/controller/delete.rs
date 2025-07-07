use crate::module::user::{dto::DeleteUserDTO, service::UserService};
use actix_web::{HttpResponse, Responder, delete, web};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[delete("")]
pub async fn delete(
    db: web::Data<DatabaseConnection>,
    req: web::Json<DeleteUserDTO>,
) -> impl Responder {
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let req = req.into_inner();
    match UserService::delete_user(db.get_ref(), req.name, req.password).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
