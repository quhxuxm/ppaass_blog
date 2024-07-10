use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use crate::entity::{LabelEntity, UserEntity};
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_label")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u32,
    #[sea_orm(primary_key)]
    pub label_id: u32,
    #[sea_orm(column_type = "Timestamp")]
    pub create_date: DateTime<Utc>,
}

#[derive(Copy, Clone, EnumIter, DeriveRelation, Debug)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::UserEntity",
        from = "Column::UserId",
        to = "crate::entity::UserColumn::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "crate::entity::LabelEntity",
        from = "Column::LabelId",
        to = "crate::entity::LabelColumn::Id"
    )]
    Label,
}

impl Related<UserEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<LabelEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
