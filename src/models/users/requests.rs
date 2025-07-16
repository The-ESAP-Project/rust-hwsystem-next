use super::entities::{UserProfile, UserRole, UserStatus};
use crate::models::common::PaginationQuery;
use serde::Deserialize;

// 用户查询参数（来自HTTP请求）
#[derive(Debug, Deserialize)]
pub struct UserListParams {
    #[serde(flatten)]
    pub pagination: PaginationQuery,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
}

// 用户创建请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub profile: UserProfile,
}

// 用户更新请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub profile: Option<UserProfile>,
}

// 用户列表查询参数（用于存储层）
#[derive(Debug, Clone, Deserialize)]
pub struct UserListQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
}
