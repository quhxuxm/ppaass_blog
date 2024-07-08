use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use uuid::Uuid;
use migration::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{
    BlogColumn, BlogEntity, PostActiveModel, PostAdditionalInfo, UserColumn, UserEntity,
};
use crate::bo::post::{CreatePostRequestBo, CreatePostResponseBo};
use crate::bo::user::UserAuthTokenBo;
use crate::error::EntryError;
use crate::extractor::auth_token::UserAuthToken;
use crate::state::ApplicationState;
pub async fn create_post(
    Path(blog_token): Path<String>,
    UserAuthToken(UserAuthTokenBo { username, .. }): UserAuthToken,
    State(state): State<ApplicationState>,
    Json(CreatePostRequestBo {
        title,
        content,
        additional_info: post_additional_info,
    }): Json<CreatePostRequestBo>,
) -> Result<Json<CreatePostResponseBo>, EntryError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(&username))
        .one(state.database())
        .await?
        .ok_or(EntryError::UserNotFoundByUsername(username.clone()))?;
    let blog_from_db = BlogEntity::find()
        .filter(
            BlogColumn::Token
                .eq(&blog_token)
                .and(BlogColumn::UserId.eq(user_from_db.id)),
        )
        .one(state.database())
        .await?
        .ok_or(EntryError::BlogNotFoundByToken(blog_token.clone()))?;
    let post_entity = PostActiveModel {
        title: Set(title),
        content: Set(Some(content)),
        create_date: Set(Utc::now()),
        update_date: Set(Utc::now()),
        blog_id: Set(blog_from_db.id),
        token: Set(Uuid::new_v4().to_string()),
        additional_info: Set(PostAdditionalInfo {
            labels: post_additional_info.labels,
        }),
        ..Default::default()
    };
    let post_from_db = post_entity.insert(state.database()).await?;
    Ok(Json(CreatePostResponseBo {
        token: post_from_db.token,
    }))
}
