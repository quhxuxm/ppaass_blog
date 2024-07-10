use axum::{debug_handler, Json};
use axum::extract::{Path, Query, State};
use ppaass_blog_persistence::dao::blog::{
    create_blog as dao_create_blog, find_all_blogs_by_username,
};
use ppaass_blog_persistence::dao::blog::get_blog as dao_get_blog_detail;
use ppaass_blog_persistence::dto::blog::CreateBlogDto;
use crate::bo::blog::{
    BlogAdditionalInfoBo, BlogDetailBo, CreateBlogRequestBo, CreateBlogResponseBo,
    ListBlogsResponseBo,
};
use crate::bo::Pagination;
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
    let blog_from_db = dao_create_blog(
        state.database(),
        CreateBlogDto {
            title,
            summary,
            username: user_auth_token.username,
            labels: additional_info.labels,
        },
    )
    .await?;
    Ok(Json(CreateBlogResponseBo {
        token: blog_from_db.token,
    }))
}

#[debug_handler]
pub async fn get_blog_detail(
    Path(blog_token): Path<String>,
    State(state): State<ApplicationState>,
) -> Result<Json<BlogDetailBo>, EntryError> {
    let blog_dto = dao_get_blog_detail(state.database(), &blog_token).await?;
    Ok(Json(BlogDetailBo {
        token: blog_dto.token,
        title: blog_dto.title,
        summary: blog_dto.summary,
        additional_info: BlogAdditionalInfoBo {
            labels: blog_dto.labels,
        },
        owner_username: blog_dto.owner_username,
    }))
}

#[debug_handler]
pub async fn list_blogs(
    Path(username): Path<String>,
    Query(Pagination {
        page_index,
        page_size,
    }): Query<Pagination>,
    State(state): State<ApplicationState>,
) -> Result<Json<ListBlogsResponseBo>, EntryError> {
    let page_index = page_index.unwrap_or(0u64);
    let page_size = page_size.unwrap_or(u64::MAX);
    let blogs =
        find_all_blogs_by_username(state.database(), username, page_index, page_size).await?;
    let blogs = blogs
        .into_iter()
        .map(|blog| BlogDetailBo {
            token: blog.token,
            title: blog.title,
            summary: blog.summary,
            additional_info: BlogAdditionalInfoBo {
                labels: blog.labels,
            },
            owner_username: blog.owner_username,
        })
        .collect();
    Ok(Json(ListBlogsResponseBo {
        blogs,
        pagination: Pagination {
            page_index: Some(page_index),
            page_size: Some(page_size),
        },
    }))
}
