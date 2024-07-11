use crate::bo::user::{
    AuthUserRequestBo, AuthUserResponseBo, GetUserResponseBo, RegisterUserRequestBo,
    RegisterUserResponseBo, UserAuthTokenBo,
};
use crate::error::EntryError;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use ppaass_blog_persistence::dao::user::{create_user, find_by_username};
use ppaass_blog_persistence::dto::user::CreateUserDto;
use std::time::Duration;
use uuid::Uuid;
#[debug_handler]
pub async fn auth_user(
    State(state): State<ApplicationState>,
    Json(AuthUserRequestBo { username, password }): Json<AuthUserRequestBo>,
) -> Result<Json<AuthUserResponseBo>, EntryError> {
    let user_from_db = find_by_username(state.database(), &username).await?;
    let Some(user_from_db) = user_from_db else {
        return Err(EntryError::UserNotFoundByUsername(username.clone()));
    };
    if !user_from_db.password.eq(&password) {
        return Err(EntryError::UserPasswordNotMatch(username.clone()));
    }
    let mut jwt_header = Header::new(Algorithm::HS512);
    jwt_header.kid = Some(Uuid::new_v4().to_string());
    let jwt_payload = UserAuthTokenBo {
        username: user_from_db.username.clone(),
        exp: (Utc::now() + Duration::from_secs(state.config().jwt().expire_duration_seconds()))
            .timestamp_millis(),
        labels: user_from_db.labels.clone(),
    };
    let jwt_encoding_key = EncodingKey::from_secret(state.config().jwt().secret().as_bytes());
    let auth_token = encode(&jwt_header, &jwt_payload, &jwt_encoding_key)
        .map_err(|e| EntryError::UserAuthToken(e.into()))?;

    Ok(Json(AuthUserResponseBo {
        username: user_from_db.username,
        display_name: user_from_db.display_name,
        labels: user_from_db.labels,
        auth_token,
    }))
}

#[debug_handler]
pub async fn get_user(
    Path(username): Path<String>,
    State(state): State<ApplicationState>,
) -> Result<Json<GetUserResponseBo>, EntryError> {
    let user_from_db = find_by_username(state.database(), &username).await?;
    let Some(user_from_db) = user_from_db else {
        return Err(EntryError::UserNotFoundByUsername(username.clone()));
    };
    Ok(Json(GetUserResponseBo {
        username: user_from_db.username,
        display_name: user_from_db.display_name,
        labels: user_from_db.labels,
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
    let user_from_db = find_by_username(state.database(), &username).await?;
    if let Some(_) = user_from_db {
        return Err(EntryError::UserExistByUsername(username));
    };
    let user_from_db = create_user(
        state.database(),
        CreateUserDto {
            username,
            password,
            display_name,
            labels,
        },
    )
    .await?;
    Ok(Json(RegisterUserResponseBo {
        username: user_from_db.username,
        display_name: user_from_db.display_name,
        labels: user_from_db.labels,
    }))
}
