use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct File {
    // 文件的唯一标识符
    pub submission_token: String,
    // 文件名称
    pub file_name: String,
    // 文件大小（以字节为单位）
    pub file_size: i64,
    // 文件类型
    pub file_type: String,
    // 上传时间
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    // 用户ID
    pub user_id: i64,
}
