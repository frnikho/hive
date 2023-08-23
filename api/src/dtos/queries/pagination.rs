use serde::{Deserialize, Serialize};
use crate::entities::pagination::Pagination;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PaginationQuery {
    pub page: Option<u16>,
    pub limit: Option<u16>,
}

impl Into<Pagination> for PaginationQuery {
    fn into(self) -> Pagination {
        Pagination {
            page: self.page.unwrap_or(0).max(0),
            limit: self.limit.unwrap_or(20).min(100),
            bypass: false
        }
    }
}