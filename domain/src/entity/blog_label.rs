use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use crate::entity::{BlogEntity, LabelEntity};
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub blog_id: u32,
    #[sea_orm(primary_key)]
    pub label_id: u32,
    #[sea_orm(column_type = "Timestamp")]
    pub create_date: DateTime<Utc>,
}

#[derive(Copy, Clone, EnumIter, DeriveRelation, Debug)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::BlogEntity",
        from = "Column::BlogId",
        to = "crate::entity::BlogColumn::Id"
    )]
    Blog,
    #[sea_orm(
        belongs_to = "crate::entity::LabelEntity",
        from = "Column::LabelId",
        to = "crate::entity::LabelColumn::Id"
    )]
    Label,
}

impl Related<BlogEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<LabelEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
