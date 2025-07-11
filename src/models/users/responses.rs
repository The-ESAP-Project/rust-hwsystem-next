use super::entities::{User, UserProfile, UserRole, UserStatus};
use crate::models::common::PaginationInfo;
use serde::Serialize;

// 用户响应模型
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub profile: UserProfile,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            status: user.status,
            profile: user.profile,
            last_login: user.last_login,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub items: Vec<User>,
    pub pagination: PaginationInfo,
}
