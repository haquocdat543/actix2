use crate::config::common::DbPool;
use crate::module::user::error::AppError;
use crate::module::user::{
    dto::CreateUserDTO,
    entity::user::{NewUser, User, Users},
    repository,
};
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

pub fn create_user(pool: &DbPool, dto: CreateUserDTO) -> Result<User, AppError> {
    // Hash the password using bcrypt
    let hashed_password = hash(dto.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: dto.name,
        email: dto.email,
        password: hashed_password,
    };

    repository::create_user(pool, new_user)
}

pub fn get_users(pool: &DbPool) -> Result<Vec<Users>, AppError> {
    repository::get_users(pool)
}
