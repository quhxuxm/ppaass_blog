use serde::{Deserialize, Serialize};
use crate::bo::Pagination;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostAdditionalInfoBo {
    pub labels: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostRequestBo {
    pub title: String,
    pub content: String,
    pub additional_info: PostAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostResponseBo {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostDetailBo {
    pub token: String,
    pub title: String,
    pub content: String,
    pub additional_info: PostAdditionalInfoBo,
    pub blog_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPostsResponseBo {
    pub posts: Vec<PostDetailBo>,
    pub pagination: Pagination,
}
