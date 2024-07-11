use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait,
    TryIntoModel,
};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{
    LabelEntity, UserLabelActiveModel, UserLabelColumn, UserLabelEntity,
};
use crate::error::DaoError;
pub async fn find_labels_by_user<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    user_id: u32,
) -> Result<Vec<String>, DaoError> {
    let labels_from_db = UserLabelEntity::find()
        .filter(UserLabelColumn::UserId.eq(user_id))
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

pub async fn save_user_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    user_id: u32,
    label_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async move {
                let user_label_from_db = UserLabelEntity::find_by_id((user_id, label_id))
                    .one(txn)
                    .await?;
                match user_label_from_db {
                    None => UserLabelActiveModel {
                        user_id: Set(user_id),
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
