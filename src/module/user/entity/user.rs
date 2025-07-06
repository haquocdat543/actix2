use serde::Serialize; // <--- add this
use sea_orm::entity::prelude::*;
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

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

