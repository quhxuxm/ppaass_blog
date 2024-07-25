use chrono::DateTime;
use chrono::Utc;
use sea_orm::prelude::*;
use crate::entity::{
    BlogEntity, BlogLabelRelation, PostEntity, PostLabelRelation, UserEntity, UserLabelRelation,
};
#[derive(DeriveEntityModel, Clone, PartialEq, Eq, Debug)]
#[sea_orm(table_name = "label")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(unique, indexed)]
    pub text: String,
    #[sea_orm(column_type = "Timestamp")]
    pub create_date: DateTime<Utc>,
}

#[derive(EnumIter, DeriveRelation, Debug, Clone, Copy)]
pub enum Relation {}

impl Related<PostEntity> for Entity {
    fn to() -> RelationDef {
        PostLabelRelation::Post.def()
    }
    fn via() -> Option<RelationDef> {
        Some(PostLabelRelation::Label.def().rev())
    }
}

impl Related<BlogEntity> for Entity {
    fn to() -> RelationDef {
        BlogLabelRelation::Blog.def()
    }
    fn via() -> Option<RelationDef> {
        Some(BlogLabelRelation::Label.def().rev())
    }
}
impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        UserLabelRelation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(UserLabelRelation::Label.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
