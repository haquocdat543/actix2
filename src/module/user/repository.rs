use super::entity::user;
use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use sea_orm::DbErr;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::{FromQueryResult, QuerySelect};
use serde::Serialize;
use uuid::Uuid; // auto-gen or manually written entity

#[derive(Debug, FromQueryResult, Serialize)]
pub struct Password {
    pub password: String,
}

#[derive(Debug, FromQueryResult, Serialize)]
pub struct UserInfo {
    email: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

pub struct UserRepository;

impl UserRepository {

    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        email: String,
        password: String,
    ) -> Result<user::Model, sea_orm::DbErr> {
        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            email: Set(email),
            password: Set(password),
            created_at: Default::default(), // Handled by `before_save`
            updated_at: Default::default(), // Handled by `before_save`
            deleted_at: Set(None),
        };
        user.insert(db).await
    }

    pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<UserInfo>, sea_orm::DbErr> {
        user::Entity::find()
            .select_only()
            .column(user::Column::Email)
            .column(user::Column::Name)
            .column(user::Column::CreatedAt)
            .column(user::Column::UpdatedAt)
            .column(user::Column::DeletedAt)
            .into_model::<UserInfo>()
            .all(db)
            .await
    }

    pub async fn get_user(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<Option<UserInfo>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.eq(name)) // ✅ filter by name
            .select_only()
            .column(user::Column::Email)
            .column(user::Column::Name)
            .column(user::Column::CreatedAt)
            .column(user::Column::UpdatedAt)
            .column(user::Column::DeletedAt)
            .into_model::<UserInfo>() // Your custom DTO
            .one(db)
            .await
    }

    pub async fn get_password(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<Option<Password>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Name.eq(name))
            .select_only()
            .column(user::Column::Password)
            .into_model::<Password>()
            .one(db)
            .await
    }

    pub async fn update_password(
        db: &DatabaseConnection,
        name: String,
        new_password: String,
    ) -> Result<bool, DbErr> {
        // 1. Find user by name
        if let Some(user) = user::Entity::find()
            .filter(user::Column::Name.eq(name))
            .one(db)
            .await?
        {
            let hashed_password = hash(&new_password, DEFAULT_COST)
                .map_err(|e| sea_orm::DbErr::Custom(format!("Hasing error {}", e)))?;

            // 3. Update fields
            let mut active: user::ActiveModel = user.into();
            active.password = Set(hashed_password);
            active.updated_at = Set(Utc::now());

            // 4. Commit update
            active.update(db).await?;

            Ok(true)
        } else {
            Ok(false) // user not found
        }
    }

    pub async fn delete_user(
        db: &DatabaseConnection,
        name: String,
    ) -> Result<bool, sea_orm::DbErr> {
        let result = user::Entity::delete_many()
            .filter(user::Column::Name.eq(name))
            .exec(db)
            .await?;

        Ok(result.rows_affected > 0)
    }

}
