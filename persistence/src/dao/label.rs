use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    TransactionTrait, TryIntoModel,
};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{LabelActiveModel, LabelColumn, LabelEntity};
use crate::error::DaoError;
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
