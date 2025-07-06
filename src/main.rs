mod config;

mod schema;

mod module {
    pub mod user {
        pub mod handler;
        pub mod dto;
        pub mod service;
        pub mod controller; // if `create.rs` is here
        pub mod entity;
        pub mod repository;
    }
}

mod router {
    pub mod router;
}

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenvy;
use env_logger::Env;
use std::env;

use crate::config::common::init_pool;
use crate::router::router::api_scope; // ✅ router/router.rs → router::router

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);
    let pool_data = web::Data::new(pool);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool_data.clone())
            .service(api_scope(pool_data.clone()))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}

