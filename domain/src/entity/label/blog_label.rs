use sea_orm::prelude::*;
#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_label")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    pub blog_id: u32,
    pub label_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::blog::Entity",
        from = "Column::BlogId",
        to = "crate::entity::blog::Column::Id"
    )]
    Blog,
    #[sea_orm(
        belongs_to = "crate::entity::label::Entity",
        from = "Column::LabelId",
        to = "crate::entity::label::Column::Id"
    )]
    Label,
}

impl Related<crate::entity::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<crate::entity::label::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Label.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
