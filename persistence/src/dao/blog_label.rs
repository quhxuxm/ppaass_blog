use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, TransactionTrait, TryIntoModel};
use sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{BlogLabelActiveModel, BlogLabelEntity};
use crate::error::DaoError;
pub async fn save_blog_label<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    label_id: u32,
    blog_id: u32,
) -> Result<(), DaoError> {
    database
        .transaction(|txn| {
            Box::pin(async {
                let blog_label_from_db = BlogLabelEntity::find_by_id((blog_id, label_id))
                    .one(txn)
                    .await?;
                match blog_label_from_db {
                    None => BlogLabelActiveModel {
                        blog_id: Set(blog_id),
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
