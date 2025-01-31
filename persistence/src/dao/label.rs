use crate::error::DaoError;
use chrono::Utc;
use ppaass_blog_domain::entity::{LabelActiveModel, LabelColumn, LabelEntity};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait,
    TryIntoModel,
};
pub async fn save_all_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    texts: Vec<String>,
) -> Result<Vec<u32>, DaoError> {
    let label_ids = database
        .transaction(|txn| {
            Box::pin(async move {
                let mut label_ids = Vec::new();
                for text in texts {
                    let label_id = save_label(txn, text).await?;
                    label_ids.push(label_id)
                }
                Ok(label_ids)
            })
        })
        .await?;
    Ok(label_ids)
}

pub async fn save_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    text: String,
) -> Result<u32, DaoError> {
    let label_model = database
        .transaction(|txn| {
            Box::pin(async move {
                let label_from_db = LabelEntity::find()
                    .filter(LabelColumn::Text.eq(&text))
                    .one(txn)
                    .await?;
                match label_from_db {
                    None => LabelActiveModel {
                        text: Set(text),
                        create_date: Set(Utc::now()),
                        ..Default::default()
                    }
                    .save(txn)
                    .await?
                    .try_into_model(),
                    Some(label_model) => Ok(label_model),
                }
            })
        })
        .await?;
    Ok(label_model.id)
}

pub async fn get_label_id<C: ConnectionTrait>(
    database: &C,
    text: String,
) -> Result<Option<u32>, DaoError> {
    Ok(LabelEntity::find()
        .filter(LabelColumn::Text.eq(&text))
        .one(database)
        .await?
        .map(|model| model.id))
}
