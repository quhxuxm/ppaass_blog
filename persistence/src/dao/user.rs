use crate::dto::user::{CreateUserDto, UpdateUserDto, UserDto};
use crate::error::DaoError;
use chrono::Utc;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TransactionTrait, TryIntoModel,
};
use ppaass_blog_domain::entity::{UserActiveModel, UserAdditionalInfo, UserColumn, UserEntity};
pub async fn find_by_username(
    database: &DatabaseConnection,
    username: &str,
) -> Result<Option<UserDto>, DaoError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(username))
        .one(database)
        .await?;
    let Some(user_from_db) = user_from_db else {
        return Ok(None);
    };
    Ok(Some(UserDto {
        username: user_from_db.username,
        password: user_from_db.password,
        display_name: user_from_db.display_name,
        labels: user_from_db.additional_info.labels,
        register_date: user_from_db.register_date,
    }))
}

pub async fn create_user(
    database: &DatabaseConnection,
    CreateUserDto {
        username,
        password,
        display_name,
        labels,
    }: CreateUserDto,
) -> Result<UserDto, DaoError> {
    let user_model = UserActiveModel {
        username: Set(username),
        display_name: Set(display_name),
        password: Set(password),
        register_date: Set(Utc::now()),
        additional_info: Set(UserAdditionalInfo { labels }),
        ..Default::default()
    };
    let user_model = database
        .transaction(|txn| Box::pin(async move { user_model.save(txn).await }))
        .await?;
    let user_model = user_model.try_into_model()?;
    Ok(UserDto {
        username: user_model.username,
        password: user_model.password,
        display_name: user_model.display_name,
        labels: user_model.additional_info.labels,
        register_date: user_model.register_date,
    })
}

pub async fn update_user(
    database: &DatabaseConnection,
    UpdateUserDto {
        username,
        password,
        display_name,
        labels,
    }: UpdateUserDto,
) -> Result<UserDto, DaoError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(&username))
        .one(database)
        .await?
        .ok_or(DaoError::UserNotFoundByUsername(username))?;
    let mut user_model = user_from_db.into_active_model();
    if let Some(display_name) = display_name {
        user_model.display_name = Set(display_name)
    }
    if let Some(password) = password {
        user_model.password = Set(password)
    }
    if let Some(labels) = labels {
        user_model.additional_info = Set(UserAdditionalInfo { labels })
    }
    let user_model = database
        .transaction(|txn| Box::pin(async move { user_model.save(txn).await }))
        .await?;
    let user_model = user_model.try_into_model()?;
    Ok(UserDto {
        username: user_model.username,
        password: user_model.password,
        display_name: user_model.display_name,
        labels: user_model.additional_info.labels,
        register_date: user_model.register_date,
    })
}
