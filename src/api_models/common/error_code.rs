pub enum ErrorCode {
    // 成功
    Success = 0, // 成功

    // 通用错误
    BadRequest = 1000,          // 错误的请求
    Unauthorized = 1001,        // 未授权访问
    NotFound = 1004,            // 未找到资源
    Conflict = 1009,            // 冲突 (资源已存在)
    InternalServerError = 1005, // 内部服务器错误

    // Auth 错误
    AuthFailed = 2000, // 身份验证失败

    // 文件相关错误
    FileNotFound = 3000,              // 文件未找到
    FileUploadFailed = 3001,          // 文件上传失败
    FileTypeNotAllowed = 3002,        // 文件类型不被允许
    FileSizeExceeded = 3003,          // 文件大小超出限制
    MuitifileUploadNotAllowed = 3004, // 不允许多文件上传
}
