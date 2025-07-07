use diesel::prelude::*;

use crate::config::common::DbPool;
use crate::module::user::entity::user::{NewUser, User, Users};
use crate::schema::user;

pub fn create_user(pool: &DbPool, new_user: NewUser) -> QueryResult<User> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(&mut conn)
}

pub fn get_users(pool: &DbPool) -> QueryResult<Vec<Users>> {
    let mut conn = pool.get().expect("Failed to get DB connection");

    user::table
        .select((
            user::email,
            user::dob,
            user::created_at,
            user::updated_at,
            user::deleted_at,
        )) // ✅ SELECT fields
        .load::<Users>(&mut conn)
}
