use entity::users;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid; // auto-gen or manually written entity

pub struct UserRepository;

impl UserRepository {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr> {
        users::Entity::find().all(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        users::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<users::Model>, sea_orm::DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        email: String,
        password: String,
    ) -> Result<users::Model, sea_orm::DbErr> {
        let user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            email: Set(email),
            password: Set(password),
            ..Default::default()
        };
        user.insert(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<(), sea_orm::DbErr> {
        users::Entity::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}
