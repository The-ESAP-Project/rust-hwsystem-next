use super::entities::Homework;
use crate::models::common::pagination::PaginationInfo;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HomeworkListResponse {
    pub items: Vec<Homework>,
    pub pagination: PaginationInfo,
}
