pub mod blog;
pub mod post;
pub mod user;

pub struct PageDto<T> {
    pub items: Vec<T>,
    pub page_index: u64,
    pub page_number: u64,
    pub page_size: u64,
}
