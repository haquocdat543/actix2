use crate::module::user::repository::UserRepository;
use entity::users;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_all_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        UserRepository::find_all(db).await
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        UserRepository::find_by_id(db, id).await
    }

    pub async fn register_user(
        db: &DatabaseConnection,
        name: String,
        email: String,
        password: String,
    ) -> Result<users::Model, sea_orm::DbErr> {
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
