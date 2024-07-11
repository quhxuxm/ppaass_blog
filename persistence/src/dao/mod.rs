pub mod blog;
pub mod blog_label;
pub mod label;
pub mod post;
pub mod post_label;
pub mod user;
pub mod user_label;
pub struct PageDto<T> {
    pub items: Vec<T>,
    pub page_index: u64,
    pub page_number: u64,
    pub page_size: u64,
}
