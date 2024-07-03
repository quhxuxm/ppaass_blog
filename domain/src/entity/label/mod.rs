use sea_orm::prelude::*;
 mod blog_label;
 mod post_label;
mod user_label;

pub use user_label::Entity as UserLabelEntity;
pub use user_label::Column as UserLabelEntityColumn;
pub use user_label::ActiveModel as UserLabelActiveModel;
pub use user_label::Model as UserLabelModel;

pub use blog_label::Entity as BlogLabelEntity;
pub use blog_label::Column as BlogLabelEntityColumn;
pub use blog_label::ActiveModel as BlogLabelActiveModel;
pub use blog_label::Model as BlogLabelModel;

pub use post_label::Entity as PostLabelEntity;
pub use post_label::Column as PostLabelEntityColumn;
pub use post_label::ActiveModel as PostLabelActiveModel;
pub use post_label::Model as PostLabelModel;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "original_label")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(unique, indexed)]
    pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq, DeriveRelation, EnumIter)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
