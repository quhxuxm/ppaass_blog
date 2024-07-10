use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, TransactionTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{UserLabelActiveModel, UserLabelEntity};
use crate::error::DaoError;
pub async fn save_user_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    label_id: u32,
    user_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async {
                let user_label_from_db = UserLabelEntity::find_by_id((user_id, label_id))
                    .one(txn)
                    .await?;
                match user_label_from_db {
                    None => UserLabelActiveModel {
                        user_id: Set(user_id),
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
