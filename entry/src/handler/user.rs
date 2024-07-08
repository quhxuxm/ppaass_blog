use std::time::Duration;
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use chrono::Utc;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use uuid::Uuid;
use migration::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{UserActiveModel, UserAdditionalInfo, UserColumn, UserEntity};
use crate::bo::user::{
    AuthUserRequestBo, AuthUserResponseBo, GetUserResponseBo, RegisterUserRequestBo,
    RegisterUserResponseBo, UserAdditionalInfoBo, UserAuthTokenBo,
};
use crate::error::EntryError;
use crate::state::ApplicationState;
#[debug_handler]
pub async fn auth_user(
    State(state): State<ApplicationState>,
    Json(AuthUserRequestBo { username, password }): Json<AuthUserRequestBo>,
) -> Result<Json<AuthUserResponseBo>, EntryError> {
    let user_from_db = UserEntity::find()
        .filter(
            UserColumn::Username
                .eq(&username)
                .and(UserColumn::Password.eq(password)),
        )
        .one(state.database())
        .await?
        .ok_or(EntryError::UserNotFoundByUsername(username.clone()))?;
    let mut jwt_header = Header::new(Algorithm::HS512);
    jwt_header.kid = Some(Uuid::new_v4().to_string());
    let jwt_payload = UserAuthTokenBo {
        username: user_from_db.username.clone(),
        exp: (Utc::now() + Duration::from_secs(state.config().jwt().expire_duration_seconds()))
            .timestamp_millis(),
        additional_info: UserAdditionalInfoBo {
            labels: user_from_db.additional_info.labels.clone(),
        },
    };
    let jwt_encoding_key = EncodingKey::from_secret(state.config().jwt().secret().as_bytes());
    let auth_token = encode(&jwt_header, &jwt_payload, &jwt_encoding_key)
        .map_err(|e| EntryError::UserAuthToken(e.into()))?;

    Ok(Json(AuthUserResponseBo {
        username: user_from_db.username,
        display_name: user_from_db.display_name,
        additional_info: UserAdditionalInfoBo {
            labels: user_from_db.additional_info.labels,
        },
        auth_token,
    }))
}

#[debug_handler]
pub async fn get_user(
    Path(username): Path<String>,
    State(state): State<ApplicationState>,
) -> Result<Json<GetUserResponseBo>, EntryError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(&username))
        .one(state.database())
        .await?
        .ok_or(EntryError::UserNotFoundByUsername(username.clone()))?;
    Ok(Json(GetUserResponseBo {
        username: user_from_db.username,
        display_name: user_from_db.display_name,
        additional_info: UserAdditionalInfoBo {
            labels: user_from_db.additional_info.labels,
        },
    }))
}

#[debug_handler]
pub async fn register_user(
    State(state): State<ApplicationState>,
    Json(RegisterUserRequestBo {
        username,
        labels,
        password,
        display_name,
    }): Json<RegisterUserRequestBo>,
) -> Result<Json<RegisterUserResponseBo>, EntryError> {
   let user_number= UserEntity::find()
        .filter(UserColumn::Username.eq(&username))
        .count(state.database())
        .await?;
    if user_number >0 {
        return Err(EntryError::UserExistByUsername(username));
    }
    let user_model = UserActiveModel {
        username: Set(username.clone()),
        display_name: Set(display_name.clone()),
        password: Set(password),
        register_date: Set(Utc::now()),
        additional_info: Set(UserAdditionalInfo {
            labels: labels.clone(),
        }),
        ..Default::default()
    };
    user_model.insert(state.database()).await?;
    let response = RegisterUserResponseBo {
        username,
        display_name,
        additional_info: UserAdditionalInfoBo { labels },
    };
    Ok(Json(response))
}
