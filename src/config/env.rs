use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let jwt = env::var("JWT").expect("JWT must be set");

        Config {
            host,
            port,
            database_url,
            jwt,
        }
    }
}
