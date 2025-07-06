use crate::module::user::handler;
use actix_web::web;
use sea_orm::DatabaseConnection;

pub fn api_scope(db: web::Data<DatabaseConnection>) -> actix_web::Scope {
    web::scope("/api")
        .app_data(web::Data::new(db.clone()))
        .service(handler::user_scope())
}
