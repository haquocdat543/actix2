use super::entity::user;
use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::{FromQueryResult, QuerySelect};
use serde::Serialize;
use uuid::Uuid; // auto-gen or manually written entity

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
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<UserInfo>, sea_orm::DbErr> {
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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        email: String,
        password: String,
    ) -> Result<user::Model, sea_orm::DbErr> {
        let hashed_password = hash(&password, DEFAULT_COST)
            .map_err(|e| sea_orm::DbErr::Custom(format!("Hashing error: {}", e)))?;

        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            email: Set(email),
            password: Set(hashed_password),
            created_at: Default::default(), // Handled by `before_save`
            updated_at: Default::default(), // Handled by `before_save`
            deleted_at: Set(None),
        };
        user.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), sea_orm::DbErr> {
        user::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
