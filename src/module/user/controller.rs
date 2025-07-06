use actix_web::{HttpResponse, Responder, get};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from declarative macro!")
}
