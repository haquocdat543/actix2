use crate::module::user::repository::UserRepository;
use super::entity::user;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<super::repository::UserInfo>, sea_orm::DbErr> {
        UserRepository::get_users(db).await
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        UserRepository::find_by_id(db, id).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        email: String,
        password: String,
    ) -> Result<user::Model, sea_orm::DbErr> {
        // Here you can add logic like:
        // - Check email existence
        // - Hash password
        // - Validate format

        // For now just call repository
        UserRepository::create(db, name, email, password).await
    }

    pub async fn remove_user(db: &DatabaseConnection, id: Uuid) -> Result<(), sea_orm::DbErr> {
        UserRepository::delete(db, id).await
    }
}
