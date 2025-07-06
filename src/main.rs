use actix::router::router;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(router::api_scope()) // Mount /api
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}
