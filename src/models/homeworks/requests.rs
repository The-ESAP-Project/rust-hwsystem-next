use super::entities::HomeworkStatus;
use crate::models::common::PaginationQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HomeworkListQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub status: Option<String>,
}
