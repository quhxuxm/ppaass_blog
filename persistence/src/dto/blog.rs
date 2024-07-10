use chrono::{DateTime, Utc};
pub struct CreateBlogDto {
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
    pub username: String,
}

pub struct UpdateBlogDto {
    pub token: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub labels: Option<Vec<String>>,
}

pub struct BlogDto {
    pub token: String,
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub owner_username: String,
}
