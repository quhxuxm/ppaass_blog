use chrono::{DateTime, Utc};
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub labels: Vec<String>,
}

pub struct UpdateUserDto {
    pub username: String,
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub labels: Option<Vec<String>>,
}

pub struct UserDto {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub labels: Vec<String>,
    pub register_date: DateTime<Utc>,
}
