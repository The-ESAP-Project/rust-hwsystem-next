use super::entities::Class;
use crate::models::common::PaginationInfo;
use serde::Serialize;

// 班级列表响应
#[derive(Debug, Serialize)]
pub struct ClassListResponse {
    pub pagination: PaginationInfo,
    pub items: Vec<Class>,
}
