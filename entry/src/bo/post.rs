use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostRequestBo {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostResponseBo {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPostByLabelsQuery {
    #[serde(default)]
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostDetailBo {
    pub token: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub labels: Vec<String>,
    pub blog_token: String,
}
