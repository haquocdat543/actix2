use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use sea_orm::{ConnectionTrait, DbErr};
use std::future::Future;
use std::pin::Pin;

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
        mut self,       // ✅ declare `mut` to allow assignment
        _db: &'life0 C, // ✅ underscore to silence warning
        insert: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Self, DbErr>> + Send + 'async_trait>>
    where
        C: ConnectionTrait + 'life0, // ✅ remove `Send + Sync + 'static`
        Self: Sized + 'async_trait,
        'life0: 'async_trait,
    {
        Box::pin(async move {
            let now = Utc::now();

            if insert {
                self.created_at = Set(now);
            }

            self.updated_at = Set(now);

            Ok(self)
        })
    }
}
