use serde::{Deserialize, Serialize};
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
