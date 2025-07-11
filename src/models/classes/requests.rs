use crate::models::common::PaginationQuery;
use serde::Deserialize;

// 班级查询参数（来自HTTP请求）
#[derive(Debug, Deserialize)]
pub struct ClassQueryParams {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub search: Option<String>,
}

// 创建班级请求
#[derive(Debug, Deserialize)]
pub struct CreateClassRequest {
    pub class_name: String,
    pub description: Option<String>,
}

// 更新班级请求
#[derive(Debug, Deserialize)]
pub struct UpdateClassRequest {
    pub class_name: Option<String>,
    pub description: Option<String>,
}

// 班级列表查询参数（用于存储层）
#[derive(Debug, Clone, Deserialize)]
pub struct ClassListQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub search: Option<String>,
}
