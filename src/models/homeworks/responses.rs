use crate::models::common::pagination::PaginationInfo;
use serde::Serialize;
use super::entities::Homework;

#[derive(Debug, Serialize)]
pub struct HomeworkListResponse {
    pub items: Vec<Homework>,
    pub pagination: PaginationInfo,
}
