use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBlogRequestBo {
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
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
    pub labels: Vec<String>,
    pub owner_username: String,
}
