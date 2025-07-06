use crate::module::user::{dto::CreateUserDTO, service::UserService};
use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::DatabaseConnection;

#[post("")]
pub async fn create(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreateUserDTO>,
) -> impl Responder {
    let req = req.into_inner();
    match UserService::register_user(db.get_ref(), req.name, req.email, req.password).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
