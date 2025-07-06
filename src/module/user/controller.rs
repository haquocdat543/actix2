use actix_web::{HttpResponse, Responder, get};

#[get("/all")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("All users")
}

