use actix_web::{get, post, delete, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use crate::module::user::{service::UserService, model::RegisterUserRequest};

#[get("/all")]
pub async fn get_all_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    match UserService::get_all_users(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

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

#[post("/")]
pub async fn register_user(
    db: web::Data<DatabaseConnection>,
    req: web::Json<RegisterUserRequest>,
) -> impl Responder {
    let req = req.into_inner();
    match UserService::register_user(db.get_ref(), req.name, req.email, req.password).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/{name}")]
pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match UserService::remove_user(db.get_ref(), name.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
