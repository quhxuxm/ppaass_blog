use crate::bo::Pagination;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogAdditionalInfoBo {
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBlogRequestBo {
    pub title: String,
    pub summary: String,
    pub additional_info: BlogAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBlogResponseBo {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogDetailBo {
    pub token: String,
    pub title: String,
    pub summary: String,
    pub additional_info: BlogAdditionalInfoBo,
    pub owner_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListBlogsResponseBo {
    pub blogs: Vec<BlogDetailBo>,
    pub pagination: Pagination,
}
