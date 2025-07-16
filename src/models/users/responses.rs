use super::entities::User;
use crate::models::common::PaginationInfo;
use serde::Serialize;

// 用户响应
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: User,
}

// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub items: Vec<User>,
    pub pagination: PaginationInfo,
}
