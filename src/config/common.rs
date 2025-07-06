use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool}; // ✅ Use Diesel's r2d2 directly

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}

