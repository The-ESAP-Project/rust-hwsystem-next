pub enum ErrorCode {
    // 成功
    Success = 0, // 成功

    // 通用错误
    BadRequest = 1000,          // 错误的请求
    Unauthorized = 1003,        // 未授权访问
    NotFound = 1004,            // 未找到资源
    Conflict = 1009,            // 冲突 (资源已存在)
    InternalServerError = 1005, // 内部服务器错误

    // Auth 错误
    AuthFailed = 2000, // 身份验证失败
}
