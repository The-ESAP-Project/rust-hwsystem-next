// 公共模块
pub mod common;

// 业务模块
pub mod users;

// 重新导出通用类型
pub use common::{ApiResponse, PaginationInfo};

// 重新导出用户相关类型
pub use users::{
    CreateUserRequest,
    UpdateUserRequest,
    // 实体
    User,
    UserListQuery,

    // 响应
    UserListResponse,
    UserProfile,

    // 请求
    UserQueryParams,
    UserRole,
    UserStatus,
};
