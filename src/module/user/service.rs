use super::entity::user;
use crate::module::user::repository::UserRepository;
use bcrypt::verify;
use sea_orm::{DatabaseConnection, DbErr};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<super::repository::UserInfo>, sea_orm::DbErr> {
        UserRepository::get_users(db).await
    }

    pub async fn login(
        db: &DatabaseConnection,
        name: String,
        password: String,
    ) -> Result<bool, DbErr> {
        let stored_password = UserRepository::get_password(db, name.clone()).await?;

        match stored_password {
            Some(hash) => {
                match verify(&password, &hash.password) {
                    Ok(true) => Ok(true),   // Password correct
                    Ok(false) => Ok(false), // Password incorrect
                    Err(e) => Err(DbErr::Custom(format!(
                        "Password verification failed: {}",
                        e
                    ))),
                }
            }
            None => Err(DbErr::RecordNotFound(format!("User '{}' not found", name))),
        }
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
