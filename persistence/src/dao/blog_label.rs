use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait,
    TryIntoModel,
};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{
    BlogLabelActiveModel, BlogLabelColumn, BlogLabelEntity, LabelEntity,
};
use crate::error::DaoError;
pub async fn find_labels_by_blog<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    blog_id: u32,
) -> Result<Vec<String>, DaoError> {
    let labels_from_db = BlogLabelEntity::find()
        .filter(BlogLabelColumn::BlogId.eq(blog_id))
        .find_also_related(LabelEntity)
        .all(database)
        .await?;
    let labels = labels_from_db
        .into_iter()
        .map_while(|(_, label_entity)| {
            let label_entity = label_entity?;
            Some(label_entity.text)
        })
        .collect();
    Ok(labels)
}

pub async fn save_blog_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    blog_id: u32,
    label_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async move {
                let blog_label_from_db = BlogLabelEntity::find_by_id((blog_id, label_id))
                    .one(txn)
                    .await?;
                match blog_label_from_db {
                    None => BlogLabelActiveModel {
                        blog_id: Set(blog_id),
                        label_id: Set(label_id),
                        create_date: Set(Utc::now()),
                    }
                    .insert(txn)
                    .await?
                    .try_into_model(),
                    Some(model) => Ok(model),
                }
            })
        })
        .await?;
    Ok(())
}
