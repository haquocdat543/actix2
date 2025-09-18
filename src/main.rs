use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use actix::config::env::Config;
use actix::router::router;

use sea_orm::{Database, DatabaseConnection};

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger initialize
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Centralized config
    let config = Config::from_env();

    // Initialize DB
    let db: DatabaseConnection = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to DB");

    let db_data = Data::new(db);
    let config_data = Data::new(config.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .service(router::api_scope(db_data.clone()))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
