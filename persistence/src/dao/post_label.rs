use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait,
    TryIntoModel,
};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{
    LabelEntity, PostLabelActiveModel, PostLabelColumn, PostLabelEntity,
};
use crate::error::DaoError;
pub async fn find_labels_by_post<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    post_id: u32,
) -> Result<Vec<String>, DaoError> {
    let labels_from_db = PostLabelEntity::find()
        .filter(PostLabelColumn::PostId.eq(post_id))
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

pub async fn save_post_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    label_id: u32,
    post_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async move {
                let post_label_from_db = PostLabelEntity::find_by_id((post_id, label_id))
                    .one(txn)
                    .await?;
                match post_label_from_db {
                    None => PostLabelActiveModel {
                        post_id: Set(post_id),
                        label_id: Set(label_id),
                        create_date: Set(Utc::now()),
                    }
                    .save(txn)
                    .await?
                    .try_into_model(),
                    Some(model) => Ok(model),
                }
            })
        })
        .await?;
    Ok(())
}
