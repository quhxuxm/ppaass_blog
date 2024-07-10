use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, TransactionTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{PostLabelActiveModel, PostLabelEntity};
use crate::error::DaoError;
pub async fn save_post_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    label_id: u32,
    post_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async {
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
