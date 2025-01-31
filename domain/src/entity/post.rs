use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use crate::entity::{BlogEntity, LabelEntity, PostLabelRelation};
#[derive(Clone, PartialEq, Eq, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(unique, indexed)]
    pub token: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub blog_id: u32,
    #[sea_orm(column_type = "Timestamp")]
    pub create_date: DateTime<Utc>,
    #[sea_orm(column_type = "Timestamp")]
    pub update_date: DateTime<Utc>,
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

impl Related<BlogEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<LabelEntity> for Entity {
    fn to() -> RelationDef {
        PostLabelRelation::Label.def()
    }
    fn via() -> Option<RelationDef> {
        Some(PostLabelRelation::Post.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
