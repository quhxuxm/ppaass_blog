use sea_orm::prelude::*;
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_label")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub post_id: u32,
    pub label_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::post::Entity",
        from = "Column::PostId",
        to = "crate::entity::post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "crate::entity::label::Entity",
        from = "Column::LabelId",
        to = "crate::entity::label::Column::Id"
    )]
    Label,
}

impl Related<crate::entity::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<crate::entity::label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
