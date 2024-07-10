use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
#[derive(Clone, PartialEq, Eq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(unique, indexed)]
    pub username: String,
    #[sea_orm(unique, indexed)]
    pub display_name: String,
    pub password: String,
    #[sea_orm(column_type = "Timestamp")]
    pub register_date: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::blog::Entity")]
    Blog,
}

impl Related<super::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
