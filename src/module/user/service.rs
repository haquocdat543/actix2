use crate::config::common::DbPool;
use crate::module::user::error::AppError;
use crate::module::user::{
    dto::{CreateUserDTO, LoginDTO},
    entity::user::{NewUser, User, Users},
    repository,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use uuid::Uuid;

pub fn create_user(pool: &DbPool, dto: CreateUserDTO) -> Result<User, AppError> {
    // Hash the password using bcrypt
    let hashed_password =
        hash(dto.password, DEFAULT_COST).map_err(|e| AppError::Internal(e.to_string()))?;

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

pub fn login(pool: &DbPool, dto: LoginDTO) -> Result<bool, AppError> {
    // 1. Get hashed password from the DB using the username
    let hashed_password = repository::get_password(pool, dto.name.clone())?;

    // 2. Compare the provided password with the hashed one using bcrypt
    match verify(dto.password, &hashed_password) {
        Ok(matching) => Ok(matching),
        Err(e) => Err(AppError::Internal(format!(
            "Password verification failed for user '{}': {}",
            dto.name, e
        ))),
    }
}
