use chrono::{DateTime, Utc};
pub struct CreatePostDto {
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
    pub blog_token: String,
}

pub struct UpdatePostDto {
    pub title: Option<String>,
    pub content: Option<String>,
    pub token: String,
    pub blog_token: String,
    pub labels: Option<Vec<String>>,
}

pub struct PostDto {
    pub token: String,
    pub title: String,
    pub content: String,
    pub labels: Vec<String>,
    pub blog_token: String,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
}
