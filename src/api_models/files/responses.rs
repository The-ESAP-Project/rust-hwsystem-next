use serde::Serialize;

/// FileAttachment
#[derive(Serialize)]
pub struct FileUploadResponse {
    /// 文件类型
    content_type: String,
    /// 下载链接
    download_url: String,
    /// 文件名
    filename: String,
    /// 文件ID
    id: String,
    /// 原始文件名
    original_name: String,
    /// 文件大小(字节)
    size: isize,
    /// 上传时间
    uploaded_at: String,
}
