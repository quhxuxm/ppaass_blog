use crate::bo::post::{
    CreatePostRequestBo, CreatePostResponseBo, PostAdditionalInfoBo, PostDetailBo,
};
use crate::bo::{PaginationRequestBo, PaginationResponseBo};
use crate::error::EntryError;
use crate::extractor::auth_token::UserAuthToken;
use crate::state::ApplicationState;
use axum::extract::{Path, Query, State};
use axum::{debug_handler, Json};
use ppaass_blog_persistence::dao::post::{
    create_post as dao_create_post, find_all_posts_by_blog_token,
};
use ppaass_blog_persistence::dao::user::find_by_username;
use ppaass_blog_persistence::dto::post::CreatePostDto;
pub async fn create_post(
    Path(blog_token): Path<String>,
    UserAuthToken(user_auth_token): UserAuthToken,
    State(state): State<ApplicationState>,
    Json(CreatePostRequestBo {
        title,
        content,
        additional_info: post_additional_info,
    }): Json<CreatePostRequestBo>,
) -> Result<Json<CreatePostResponseBo>, EntryError> {
    let user_from_db = find_by_username(state.database(), &user_auth_token.username).await?;
    let Some(user_from_db) = user_from_db else {
        return Err(EntryError::UserNotFoundByUsername(user_auth_token.username));
    };
    let post_dto = dao_create_post(
        state.database(),
        CreatePostDto {
            title,
            content,
            labels: post_additional_info.labels,
            blog_token,
        },
    )
    .await?;

    Ok(Json(CreatePostResponseBo {
        token: post_dto.token,
    }))
}

#[debug_handler]
pub async fn list_posts(
    Path(blog_token): Path<String>,
    Query(PaginationRequestBo {
        page_index,
        page_size,
    }): Query<PaginationRequestBo>,
    State(state): State<ApplicationState>,
) -> Result<Json<PaginationResponseBo<PostDetailBo>>, EntryError> {
    let page_index = page_index.unwrap_or(0u64);
    let page_size = page_size.unwrap_or(u64::MAX);
    let post_page =
        find_all_posts_by_blog_token(state.database(), blog_token, page_index, page_size).await?;
    let posts = post_page
        .items
        .into_iter()
        .map(|post| PostDetailBo {
            token: post.token,
            title: post.title,
            content: post.content,
            additional_info: PostAdditionalInfoBo {
                labels: post.labels,
            },
            blog_token: post.blog_token,
        })
        .collect();
    Ok(Json(PaginationResponseBo {
        items: posts,
        page_number: post_page.page_number,
        page_index: post_page.page_index,
        page_size: post_page.page_size,
    }))
}
