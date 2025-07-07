use crate::module::user::{dto::LoginDTO, service::UserService};
use crate::share::jwt::generate_token;

use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[post("/login")]
pub async fn login(db: web::Data<DatabaseConnection>, req: web::Json<LoginDTO>) -> impl Responder {
    // Validate input
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let req = req.into_inner();

    match UserService::login(db.get_ref(), req.name.clone(), req.password.clone()).await {
        Ok(true) => match generate_token(&req.name) {
            Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
            Err(e) => HttpResponse::InternalServerError().body(format!("Token error: {}", e)),
        },

        Ok(false) => {
            let login_status = Response {
                message: "Login failed".to_string(),
            };
            HttpResponse::Unauthorized().json(login_status)
        }

        Err(err) => {
            let error_response = Response {
                message: err.to_string(),
            };
            HttpResponse::BadRequest().json(error_response)
        }
    }
}
