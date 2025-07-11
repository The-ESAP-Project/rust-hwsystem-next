use crate::models::users::entities::User;
use serde::Serialize;

// 用户响应模型
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub user: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
