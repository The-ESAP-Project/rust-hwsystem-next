use serde::Serialize;

use crate::models::{PaginationInfo, class_student::entities::ClassStudent};

/// 班级学生列表响应
#[derive(Debug, Serialize)]
pub struct ClassStudentListResponse {
    pub pagination: PaginationInfo,
    pub items: Vec<ClassStudent>,
}
