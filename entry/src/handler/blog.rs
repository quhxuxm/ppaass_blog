use std::ops::Deref;
use std::sync::Arc;
use axum::{debug_handler, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use tracing::error;
use uuid::Uuid;
use migration::sea_orm::{ActiveModelTrait, DatabaseConnection};
use migration::sea_orm::ActiveValue::Set;
use ppaass_blog_domain::entity::{BlogActiveModel, BlogAdditionalInfo};
use crate::bo::blog::{
    BlogAdditionalInfoBo, CreateBlogRequestBo, CreateBlogResponseBo, GetBlogResponseBo,
};
#[debug_handler]
pub async fn create_blog(
    State(database): State<Arc<DatabaseConnection>>,
    Json(CreateBlogRequestBo {
        title,
        summary,
        additional_info,
    }): Json<CreateBlogRequestBo>,
) -> Result<Json<CreateBlogResponseBo>, StatusCode> {
    let blog = BlogActiveModel {
        title: Set(title),
        token: Set(Uuid::new_v4().to_string()),
        summary: Set(summary),
        additional_info: Set(BlogAdditionalInfo {
            labels: additional_info.labels,
        }),
        ..Default::default()
    };
    let blog_from_db = blog.insert(database.deref()).await.map_err(|e| {
        error!("Fail to insert blog to database because of error: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(CreateBlogResponseBo {
        token: blog_from_db.token,
        title: blog_from_db.title,
        summary: blog_from_db.summary,
        additional_info: BlogAdditionalInfoBo {
            labels: blog_from_db.additional_info.labels,
        },
    }))
}

pub async fn get_blog(
    Path(blog_token): Path<String>,
    State(database): State<Arc<DatabaseConnection>>,
) -> Result<Json<GetBlogResponseBo>, StatusCode> {
    todo!()
}
