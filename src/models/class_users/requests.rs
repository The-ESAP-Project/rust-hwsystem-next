use crate::models::{class_users::entities::ClassUserRole, common::PaginationQuery};
use serde::Deserialize;

// 加入班级请求
#[derive(Debug, Deserialize)]
pub struct JoinClassRequest {
    pub invite_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassUserRequest {
    pub role: Option<ClassUserRole>, // 更新用户角色
}

#[derive(Debug, Deserialize)]
pub struct ClassUserListParams {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub search: Option<String>,
}

// 班级列表查询参数（用于存储层）
#[derive(Debug, Clone, Deserialize)]
pub struct ClassUserQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub search: Option<String>,
}
