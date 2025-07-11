use crate::models::common::pagination::PaginationQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HomeworkListQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub status: Option<String>,
    pub search: Option<String>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}
