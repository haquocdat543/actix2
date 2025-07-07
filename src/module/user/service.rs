use crate::config::common::DbPool;
use crate::module::user::error::AppError;
use crate::module::user::{
    dto::{CreateUserDTO, LoginDTO},
    entity::user::{NewUser, User, Users},
    repository,
};
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng; // ✅ proper import

use uuid::Uuid;

pub fn create_user(pool: &DbPool, dto: CreateUserDTO) -> Result<User, AppError> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create an Argon2 hasher
    let argon2 = Argon2::default();

    // Hash the password with Argon2
    let hashed_password = argon2
        .hash_password(dto.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string(); // Save the full hash string (with salt, parameters, etc.)

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
    // 1. Get the hashed password from DB
    let hashed_password = repository::get_password(pool, dto.name.clone())?;

    // 2. Parse the Argon2 hash string
    let parsed_hash = PasswordHash::new(&hashed_password)
        .map_err(|e| AppError::Internal(format!("Hash parsing failed: {}", e)))?;

    // 3. Verify the provided password
    let verified = Argon2::default()
        .verify_password(dto.password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(verified)
}
