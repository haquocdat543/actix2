#[inline]
pub fn get_env(env_name: &str) -> String {
    std::env::var(env_name)
        .unwrap_or_else(|_| panic!("[{}] environment variable must be set", env_name))
}

#[inline]
pub fn get_env_host() -> String {
    get_env("HOST")
}

#[inline]
pub fn get_env_port() -> String {
    get_env("PORT")
}

#[inline]
pub fn get_env_database_url() -> String {
    get_env("DATABASE_URL")
}

#[inline]
pub fn get_env_jwt() -> String {
    get_env("JWT")
}
