use serde::Serialize;

use crate::models::{PaginationInfo, class_users::entities::ClassUser};

/// 班级学生列表响应
#[derive(Debug, Serialize)]
pub struct ClassUserListResponse {
    pub pagination: PaginationInfo,
    pub items: Vec<ClassUser>,
}
