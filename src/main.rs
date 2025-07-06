use actix::router::router;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use sea_orm::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let db_data = Data::new(db); // ✅ Wrap it here

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // ✅ this is from actix_web::middleware
            .app_data(db_data.clone()) // ✅ Share it with app
            .service(router::api_scope(db_data.clone())) // Mount /api
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
