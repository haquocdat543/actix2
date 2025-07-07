use diesel::prelude::*;

use crate::config::common::DbPool;
use crate::module::user::entity::user::{NewUser, User, Users};
use crate::module::user::error::AppError;
use crate::schema::user;

pub fn create_user(pool: &DbPool, new_user: NewUser) -> Result<User, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(&mut conn)
        .map_err(AppError::from)
}

pub fn get_users(pool: &DbPool) -> Result<Vec<Users>, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    user::table
        .select((
            user::email,
            user::dob,
            user::created_at,
            user::updated_at,
            user::deleted_at,
        )) // ✅ SELECT fields
        .load::<Users>(&mut conn)
        .map_err(AppError::from)
}

pub fn get_password(pool: &DbPool, name: String) -> Result<String, AppError> {
    let mut conn = pool.get().map_err(|e| AppError::Internal(e.to_string()))?;

    user::table
        .filter(user::name.eq(name))
        .select(user::password) // ✅ SELECT fields
        .first::<String>(&mut conn)
        .map_err(AppError::from)
}
