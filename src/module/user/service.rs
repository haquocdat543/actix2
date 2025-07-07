use super::entity::user;
use crate::module::user::repository::UserRepository;
use bcrypt::verify;
use sea_orm::{DatabaseConnection, DbErr};

pub struct UserService;

impl UserService {

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

    pub async fn get_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<super::repository::UserInfo>, sea_orm::DbErr> {
        UserRepository::get_users(db).await
    }

    pub async fn get_user(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<Option<super::repository::UserInfo>, sea_orm::DbErr> {
        UserRepository::get_user(db, name).await
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

    pub async fn update_password(
        db: &DatabaseConnection,
        name: String,
        password: String,
        new_password: String,
    ) -> Result<bool, DbErr> {
        let stored_password = UserRepository::get_password(db, name.clone()).await?;

        match stored_password {
            Some(hash) => {
                match verify(&password, &hash.password) {
                    Ok(true) => {
                        // Step 3: If valid, delete user
                        match UserRepository::update_password(db, name, new_password).await {
                            Ok(updated) => Ok(updated),
                            Err(e) => Err(e),
                        }
                    }
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

    pub async fn delete_user(
        db: &DatabaseConnection,
        name: String,
        password: String,
    ) -> Result<bool, DbErr> {
        let stored_password = UserRepository::get_password(db, name.clone()).await?;

        match stored_password {
            Some(hash) => {
                match verify(&password, &hash.password) {
                    Ok(true) => {
                        // Step 3: If valid, delete user
                        match UserRepository::delete_user(db, name).await {
                            Ok(deleted) => Ok(deleted),
                            Err(e) => Err(e),
                        }
                    }
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

}
