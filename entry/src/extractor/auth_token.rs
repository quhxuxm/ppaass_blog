use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use tracing::{debug, error};
use crate::bo::user::UserAuthTokenBo;
use crate::error::{UserAuthTokenError};
use crate::state::ApplicationState;
pub struct UserAuthToken(pub UserAuthTokenBo);

#[async_trait::async_trait]
impl FromRequestParts<ApplicationState> for UserAuthToken {
    type Rejection = UserAuthTokenError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApplicationState,
    ) -> Result<Self, Self::Rejection> {
        let authorization_header = parts.headers.get(http::header::AUTHORIZATION);
        let Some(authorization_header_value) = authorization_header else {
            return Err(UserAuthTokenError::AuthTokenNotExist);
        };
        let authorization_header_value = authorization_header_value.to_str().map_err(|e| {
            error!("Fail to decode auth token because of error: {e:?}");
            UserAuthTokenError::AuthTokenInvalid
        })?;
        let authorization_header_value_parts = authorization_header_value
            .split_whitespace()
            .collect::<Vec<&str>>();
        debug!("Get authorization header: {authorization_header_value_parts:?}");
        if authorization_header_value_parts.len() < 2 {
            return Err(UserAuthTokenError::AuthTokenInvalid);
        }
        let auth_token = authorization_header_value_parts[1];
        debug!("Get jwt token: {auth_token}");
        let jwt_decode_key = DecodingKey::from_secret(state.config().jwt().secret().as_bytes());
        let jwt_token = decode::<UserAuthTokenBo>(
            auth_token,
            &jwt_decode_key,
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|e| {
            error!("Fail to decode auth token because of error: {e:?}");
            UserAuthTokenError::AuthTokenInvalid
        })?;
        let user_auth_token_bo = jwt_token.claims;
        Ok(Self(user_auth_token_bo))
    }
}
