use crate::config::common::DbPool;
use crate::module::user::{
    dto::CreateUserDTO,
    entity::user::{NewUser, User, Users},
    repository,
};
use uuid::Uuid;

pub fn create_user(pool: &DbPool, dto: CreateUserDTO) -> User {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: dto.name,
        email: dto.email,
        password: dto.password,
    };

    repository::create_user(pool, new_user).expect("Failed to create user")
}

pub fn get_users(pool: &DbPool) -> Vec<Users> {
    repository::get_users(pool).expect("Failed to get users")
}
