use crate::module::user::{dto::LoginDTO, service::UserService};
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
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let req = req.into_inner();
    match UserService::login(db.get_ref(), req.name, req.password).await {
        Ok(user) => match user {
            true => {
                let login_status = Response {
                    message: "Login successfully".to_string(),
                };

                HttpResponse::Created().json(login_status)
            }
            false => {
                let login_status = Response {
                    message: "Login failed".to_string(),
                };

                HttpResponse::Created().json(login_status)
            }
        },
        Err(err) => {
            let error_response = Response {
                message: err.to_string(),
            };

            HttpResponse::BadRequest().json(error_response)
        }
    }
}
