use chrono::Utc;
use sea_orm::ConnectionTrait;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set,
    TransactionTrait, TryIntoModel,
};
use ppaass_blog_domain::entity::{UserActiveModel, UserColumn, UserEntity};
use crate::dao::label::save_label;
use crate::dao::user_label::{find_labels_by_user, save_user_label};
use crate::dto::user::{CreateUserDto, UpdateUserDto, UserDto};
use crate::error::DaoError;
pub async fn find_by_username<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    username: &str,
) -> Result<Option<UserDto>, DaoError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(username))
        .one(database)
        .await?;
    let Some(user_from_db) = user_from_db else {
        return Ok(None);
    };
    let labels = find_labels_by_user(database, user_from_db.id).await?;
    Ok(Some(UserDto {
        username: user_from_db.username,
        password: user_from_db.password,
        display_name: user_from_db.display_name,
        labels,
        register_date: user_from_db.register_date,
    }))
}

pub async fn create_user<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    CreateUserDto {
        username,
        password,
        display_name,
        labels,
    }: CreateUserDto,
) -> Result<UserDto, DaoError> {
    let labels_clone = labels.clone();
    let user_model = database
        .transaction(|txn| {
            Box::pin(async move {
                let user_model = UserActiveModel {
                    username: Set(username),
                    display_name: Set(display_name),
                    password: Set(password),
                    register_date: Set(Utc::now()),
                    ..Default::default()
                };
                let user_model = user_model.save(txn).await?;
                let user_model = user_model.try_into_model()?;
                let mut label_ids_from_db = Vec::new();
                for text in labels {
                    let label_id_from_db = save_label(txn, text).await?;
                    label_ids_from_db.push(label_id_from_db);
                }
                for label_id_from_db in label_ids_from_db {
                    save_user_label(txn, user_model.id, label_id_from_db).await?;
                }
                Ok(user_model)
            })
        })
        .await?;

    Ok(UserDto {
        username: user_model.username,
        password: user_model.password,
        display_name: user_model.display_name,
        labels: labels_clone,
        register_date: user_model.register_date,
    })
}

pub async fn update_user<C: ConnectionTrait + TransactionTrait>(
    database: &C,
    UpdateUserDto {
        username,
        password,
        display_name,
        labels,
    }: UpdateUserDto,
) -> Result<UserDto, DaoError> {
    let (user_model, labels) = database
        .transaction(|txn| {
            Box::pin(async move {
                let user_from_db = UserEntity::find()
                    .filter(UserColumn::Username.eq(&username))
                    .one(txn)
                    .await?
                    .ok_or(DaoError::UserNotFoundByUsername(username))?;
                let user_id = user_from_db.id;
                let mut user_model = user_from_db.into_active_model();
                if let Some(display_name) = display_name {
                    user_model.display_name = Set(display_name)
                }
                if let Some(password) = password {
                    user_model.password = Set(password)
                }
                if let Some(labels) = labels {
                    let mut label_ids_from_db = Vec::new();
                    for text in labels {
                        let label_id_from_db = save_label(txn, text).await?;
                        label_ids_from_db.push(label_id_from_db);
                    }
                    for label_id in label_ids_from_db {
                        save_user_label(txn, label_id, user_id).await?;
                    }
                }
                let user_model = user_model.save(txn).await?;
                let user_model = user_model.try_into_model()?;
                let labels = find_labels_by_user(txn, user_model.id).await?;
                Ok((user_model, labels))
            })
        })
        .await?;

    Ok(UserDto {
        username: user_model.username,
        password: user_model.password,
        display_name: user_model.display_name,
        labels,
        register_date: user_model.register_date,
    })
}
