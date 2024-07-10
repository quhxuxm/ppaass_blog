use sea_orm::prelude::*;
#[derive(DeriveEntityModel, Clone, PartialEq, Eq, Debug)]
#[sea_orm(table_name = "label")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(unique, indexed)]
    pub text: String,
}

#[derive(EnumIter, DeriveRelation, Debug, Clone, Copy)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
