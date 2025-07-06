use actix::router::router;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
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

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // ✅ Share it with app
            .service(router::api_scope(db_data.clone())) // Mount /api
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
