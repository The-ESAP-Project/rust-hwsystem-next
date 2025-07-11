use serde::Serialize;

/// FileAttachment
#[derive(Serialize)]
pub struct FileUploadResponse {
    /// 文件名
    pub submission_token: String,
    /// 原始文件名
    pub file_name: String,
    /// 文件大小(字节)
    pub size: i64,
    /// 文件类型
    pub content_type: String,
    /// 上传时间
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}
