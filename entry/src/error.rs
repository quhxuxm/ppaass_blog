use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::Error as JwtError;
use ppaass_blog_persistence::error::DaoError;
use thiserror::Error;
use tracing::error;
#[derive(Error, Debug)]
pub enum UserAuthTokenError {
    #[error("Authentication token not exist")]
    AuthTokenNotExist,
    #[error("Authentication token invalid")]
    AuthTokenInvalid,
    #[error("Authentication token expired")]
    AuthTokenExpired,
    #[error("Fail to generate auth token because of error: {0:?}")]
    AuthTokenGenerationFail(#[from] JwtError),
}

impl IntoResponse for UserAuthTokenError {
    fn into_response(self) -> Response {
        match self {
            UserAuthTokenError::AuthTokenNotExist => {
                (StatusCode::UNAUTHORIZED, "Authentication token not exist").into_response()
            }
            UserAuthTokenError::AuthTokenInvalid => {
                (StatusCode::UNAUTHORIZED, "Authentication token invalid").into_response()
            }
            UserAuthTokenError::AuthTokenExpired => {
                (StatusCode::UNAUTHORIZED, "Authentication token expired").into_response()
            }
            UserAuthTokenError::AuthTokenGenerationFail(e) => {
                error!("Fail to generate auth token because of error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Fail to generate auth token.",
                )
                    .into_response()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum EntryError {
    #[error(transparent)]
    UserAuthToken(#[from] UserAuthTokenError),
    #[error("Can not find user by username [{0}]")]
    UserNotFoundByUsername(String),
    #[error("User password not match, username: [{0}]")]
    UserPasswordNotMatch(String),
    #[error("User exist by username [{0}]")]
    UserExistByUsername(String),
    #[error("Can not find blog by token [{0}]")]
    BlogNotFoundByToken(String),
    #[error("Dao error happen: {0:?}")]
    Dao(#[from] DaoError),
}

impl IntoResponse for EntryError {
    fn into_response(self) -> Response {
        match self {
            EntryError::UserAuthToken(auth_token_error) => auth_token_error.into_response(),
            EntryError::UserNotFoundByUsername(username) => {
                error!("Can not find user from database by username: [{username}]");
                (StatusCode::NOT_FOUND, "Can not find user by username.").into_response()
            }
            EntryError::UserExistByUsername(username) => {
                error!("User existing by username: [{username}]");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "User existing by username.",
                )
                    .into_response()
            }
            EntryError::UserPasswordNotMatch(username) => {
                error!("User password not match, username: [{username}]");
                (StatusCode::UNAUTHORIZED, "User password not match.").into_response()
            }
            EntryError::BlogNotFoundByToken(token) => {
                error!("Can not find blog by token: [{token}]");
                (StatusCode::NOT_FOUND, "Can not find blog by token.").into_response()
            }
            EntryError::Dao(e) => {
                error!("Dao error happen: {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Dao error happen.").into_response()
            }
        }
    }
}
