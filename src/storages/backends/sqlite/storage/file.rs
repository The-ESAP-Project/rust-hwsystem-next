use sqlx::Row;

use super::SqliteStorage;
use crate::api_models::files::entities::File;

use crate::errors::{HWSystemError, Result};

pub async fn upload_file(
    storage: &SqliteStorage,
    submission_token: &str,
    file_name: &str,
    file_size: &i64,
    file_type: &str,
    user_id: i64,
) -> Result<File> {
    let now = chrono::Utc::now();

    let _ = sqlx::query(
    "INSERT INTO files (submission_token, file_name, file_size, file_type, uploaded_at, user_id)
        VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(submission_token)
    .bind(file_name)
    .bind(file_size)
    .bind(file_type)
    .bind(now.timestamp())
    .bind(user_id)
    .execute(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("上传文件失败: {e}")))?;

    Ok(File {
        submission_token: submission_token.to_string(),
        file_name: file_name.to_string(),
        file_size: *file_size,
        file_type: file_type.to_string(),
        uploaded_at: now,
        user_id,
    })
}

pub async fn get_file_by_token(storage: &SqliteStorage, file_id: String) -> Result<Option<File>> {
    let result = sqlx::query("SELECT * FROM files WHERE submission_token = ?")
        .bind(file_id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询文件失败: {e}")))?;

    match result {
        Some(row) => {
            let submission_token: String = row.get("submission_token");
            let file_name: String = row.get("file_name");
            let file_size: i64 = row.get("file_size");
            let file_type: String = row.get("file_type");
            let uploaded_at_ts: i64 = row.get("uploaded_at");

            let uploaded_at = chrono::DateTime::from_timestamp(uploaded_at_ts, 0)
                .ok_or_else(|| HWSystemError::date_parse("无效的上传时间戳".to_string()))?;

            let user_id: i64 = row.get("user_id");

            Ok(Some(File {
                submission_token,
                file_name,
                file_size,
                file_type,
                uploaded_at,
                user_id,
            }))
        }
        None => Ok(None),
    }
}
