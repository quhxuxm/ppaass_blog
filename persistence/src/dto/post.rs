use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
pub struct CreatePostDto {
    pub title: String,
    pub summary: String,
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

#[derive(FromQueryResult)]
pub struct PostDto {
    pub token: String,
    pub title: String,
    pub content: String,
    pub summary: String,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub blog_token: String,
}
