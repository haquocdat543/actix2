use actix::module::user::controller;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(controller::hello))
        .bind("127.0.0.1:8089")?
        .run()
        .await
}
