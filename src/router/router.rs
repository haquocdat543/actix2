use actix_web::{web, Scope};
use crate::config::common::DbPool;
use crate::module::user::handler;

pub fn api_scope(db: web::Data<DbPool>) -> Scope {
    web::scope("/api")
        .app_data(db.clone()) // ✅ share the existing Data<DbPool>
        .service(handler::user_scope())
}

