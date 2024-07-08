use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use chrono::Utc;
use uuid::Uuid;
use migration::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{
    BlogActiveModel, BlogAdditionalInfo, BlogColumn, BlogEntity, UserColumn, UserEntity,
};
use crate::bo::blog::{
    BlogAdditionalInfoBo, CreateBlogRequestBo, CreateBlogResponseBo, GetBlogResponseBo,
};
use crate::error::EntryError;
use crate::extractor::auth_token::UserAuthToken;
use crate::state::ApplicationState;
#[debug_handler]
pub async fn create_blog(
    State(state): State<ApplicationState>,
    UserAuthToken(user_auth_token): UserAuthToken,
    Json(CreateBlogRequestBo {
        title,
        summary,
        additional_info,
    }): Json<CreateBlogRequestBo>,
) -> Result<Json<CreateBlogResponseBo>, EntryError> {
    let user_from_db = UserEntity::find()
        .filter(UserColumn::Username.eq(&user_auth_token.username))
        .one(state.database())
        .await?
        .ok_or(EntryError::UserNotFoundByUsername(
            user_auth_token.username.clone(),
        ))?;

    let blog = BlogActiveModel {
        title: Set(title),
        token: Set(Uuid::new_v4().to_string()),
        summary: Set(summary),
        user_id: Set(user_from_db.id),
        additional_info: Set(BlogAdditionalInfo {
            labels: additional_info.labels,
        }),
        create_date: Set(Utc::now()),
        update_date: Set(Utc::now()),
        ..Default::default()
    };
    let blog_from_db = blog.insert(state.database()).await?;
    Ok(Json(CreateBlogResponseBo {
        token: blog_from_db.token,
    }))
}

pub async fn get_blog(
    Path(blog_token): Path<String>,
    State(state): State<ApplicationState>,
) -> Result<Json<GetBlogResponseBo>, EntryError> {
    let blog_from_db = BlogEntity::find()
        .filter(BlogColumn::Token.eq(&blog_token))
        .one(state.database())
        .await?
        .ok_or(EntryError::BlogNotFoundByToken(blog_token.clone()))?;
    Ok(Json(GetBlogResponseBo {
        token: blog_token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        additional_info: BlogAdditionalInfoBo {
            labels: blog_from_db.additional_info.labels,
        },
    }))
}
