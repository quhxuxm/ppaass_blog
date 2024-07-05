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
    pub title: String,
    pub summary: String,
    pub additional_info: BlogAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBlogResponseBo {
    pub token: String,
    pub title: String,
    pub summary: String,
    pub additional_info: BlogAdditionalInfoBo,
}
