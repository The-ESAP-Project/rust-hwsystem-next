use serde::{Deserialize, Serialize};

use crate::models::ErrorCode;

// 统一的API响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::Success as i32,
            message: message.into(),
            data: Some(data),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl ApiResponse<()> {
    pub fn success_empty(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::Success as i32,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error_empty(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code as i32,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now(),
        }
    }
}
