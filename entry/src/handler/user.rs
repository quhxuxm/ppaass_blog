use std::ops::Deref;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use log::error;
use migration::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{UserActiveModel, UserAdditionalInfo, UserColumn, UserEntity};
use crate::bo::user::{
    GetUserResponseBo, RegisterUserRequestBo, RegisterUserResponseBo, UserAdditionalInfoBo,
};
pub async fn get(
    Path(username): Path<String>,
    State(database): State<Arc<DatabaseConnection>>,
) -> Result<GetUserResponseBo, StatusCode> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::UserName.eq(&username))
        .one(database.deref())
        .await
        .map_err(|e| {
            error!(
                "Fail to find user from database with username: {username} because of error: {e:?}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(GetUserResponseBo {
        user_name: user_from_db.user_name,
        display_name: user_from_db.display_name,
        additional_info: UserAdditionalInfoBo {
            labels: user_from_db.additional_info.labels,
        },
    })
}

pub async fn register(
    State(database): State<Arc<DatabaseConnection>>,
    Json(RegisterUserRequestBo {
        user_name,
        labels,
        password,
        display_name,
    }): Json<RegisterUserRequestBo>,
) -> Result<Json<RegisterUserResponseBo>, StatusCode> {
    let user_model = UserActiveModel {
        user_name: Set(user_name.clone()),
        display_name: Set(display_name.clone()),
        password: Set(password),
        register_date: Set(Utc::now()),
        additional_info: Set(UserAdditionalInfo {
            labels: labels.clone(),
        }),
        ..Default::default()
    };
    user_model.insert(database.deref()).await.map_err(|e| {
        error!("Fail to insert user into database because of error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let response = RegisterUserResponseBo {
        user_name,
        display_name,
        additional_info: UserAdditionalInfoBo { labels },
    };
    Ok(Json(response))
}
