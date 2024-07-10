use serde::{Deserialize, Serialize};
pub mod blog;
pub mod post;
pub mod user;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pagination {
    pub page_index: Option<u64>,
    pub page_size: Option<u64>,
}
