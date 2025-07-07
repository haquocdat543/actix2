use bcrypt::hash;
use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use sea_orm::{ConnectionTrait, DbErr};
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(unique)]
    pub name: String,

    #[sea_orm(unique)]
    pub email: String,

    pub password: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save<'life0, 'async_trait, C>(
        mut self,
        _db: &'life0 C,
        insert: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Self, DbErr>> + Send + 'async_trait>>
    where
        C: ConnectionTrait + 'life0,
        Self: Sized + 'async_trait,
        'life0: 'async_trait,
    {
        Box::pin(async move {
            let now = Utc::now();

            if insert {
                // ✅ Extract inner string from ActiveValue
                let raw_password = self
                    .password
                    .take()
                    .ok_or_else(|| DbErr::Custom("Password must be set".into()))?;

                // ✅ bcrypt works with &str or String
                let hashed_password = hash(&raw_password, 10)
                    .map_err(|e| DbErr::Custom(format!("Hashing error: {}", e)))?;

                // ✅ Set the hashed password back into the model
                self.password = Set(hashed_password);
                self.created_at = Set(now);
            }

            let raw_password = self
                .password
                .take()
                .ok_or_else(|| DbErr::Custom("Password must be set".into()))?;

            // ✅ bcrypt works with &str or String
            let hashed_password = hash(&raw_password, 10)
                .map_err(|e| DbErr::Custom(format!("Hashing error: {}", e)))?;

            // ✅ Set the hashed password back into the model
            self.password = Set(hashed_password);
            self.updated_at = Set(now);
            Ok(self)
        })
    }
}
