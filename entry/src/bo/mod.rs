use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub mod blog;
pub mod post;
pub mod user;

#[derive(Deserialize, Clone, Debug)]
pub struct PaginationRequestBo {
    pub page_index: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PaginationResponseBo<T>
where
    T: Serialize + Clone + Debug,
{
    pub page_index: u64,
    pub page_size: u64,
    pub page_number: u64,
    pub items: Vec<T>,
}
