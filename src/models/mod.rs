// 公共模块
pub mod common;

// 登录模块
pub mod auth;

// 业务模块
pub mod users;

// 文件模块
pub mod files;
pub mod homeworks;

// 重新导出通用类型
pub use common::{ApiResponse, ErrorCode, PaginationInfo};

// 系统模块
#[derive(Clone, Debug)]
pub struct AppStartTime {
    pub start_datetime: chrono::DateTime<chrono::Utc>,
}
