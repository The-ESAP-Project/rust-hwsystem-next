use serde::Deserialize;
use crate::models::common::PaginationQuery;

#[derive(Debug, Deserialize)]
pub struct HomeworkListQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub status: Option<String>,
}
