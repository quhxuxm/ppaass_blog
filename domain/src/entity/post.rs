use chrono::{DateTime, Utc};
use sea_orm::FromJsonQueryResult;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, FromJsonQueryResult, Clone, Eq, PartialEq, Debug)]
pub struct AdditionalInfo {
    pub labels: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub title: String,
    #[sea_orm(nullable)]
    pub content: Option<String>,
    pub blog_id: u32,
    #[sea_orm(column_type = "Timestamp")]
    pub create_date: DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub update_date: DateTime<Utc>,
    pub additional_info: AdditionalInfo
}

#[derive(DeriveRelation, Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::blog::Entity",
        from = "Column::BlogId",
        to = "super::blog::Column::Id"
    )]
    Blog,
}

impl Related<super::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
