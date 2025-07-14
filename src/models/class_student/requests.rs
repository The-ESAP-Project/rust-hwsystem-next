use crate::models::{class_student::entities::ClassUserRole, common::PaginationQuery};
use serde::Deserialize;

// 加入班级请求
#[derive(Debug, Deserialize)]
pub struct JoinClassRequest {
    pub invite_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentRequest {
    pub role: ClassUserRole, // 更新学生角色
}

#[derive(Debug, Deserialize)]
pub struct ClassStudentQueryParams {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub search: Option<String>,
}

// 班级列表查询参数（用于存储层）
#[derive(Debug, Clone, Deserialize)]
pub struct ClassStudentQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub teacher_id: Option<i64>,
    pub search: Option<String>,
}
