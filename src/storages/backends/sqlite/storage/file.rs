use sqlx::Row;

use super::SqliteStorage;
use crate::api_models::files::entities::File;

use crate::errors::{HWSystemError, Result};

pub async fn upload_file(
    storage: &SqliteStorage,
    unique_name: &str,
    file_name: &str,
    file_size: &i64,
    file_type: &str,
    user_id: i64,
) -> Result<File> {
    let now = chrono::Utc::now();

    let result = sqlx::query(
        "INSERT INTO files (unique_name, file_name, file_size, file_type, uploaded_at, user_id)
            VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
    )
    .bind(unique_name)
    .bind(file_name)
    .bind(file_size)
    .bind(file_type)
    .bind(now.timestamp()) // 使用时间戳
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("上传文件失败: {e}")))?;

    let id: i64 = result.get("id");

    Ok(File {
        id,
        unique_name: unique_name.to_string(),
        file_name: file_name.to_string(),
        file_size: *file_size,
        file_type: file_type.to_string(),
        uploaded_at: now,
        user_id,
    })
}

pub async fn get_file_by_id(storage: &SqliteStorage, file_id: i64) -> Result<Option<File>> {
    let result = sqlx::query("SELECT * FROM files WHERE id = ?")
        .bind(file_id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询文件失败: {e}")))?;

    match result {
        Some(row) => {
            let id: i64 = row.get("id");
            let unique_name: String = row.get("unique_name");
            let file_name: String = row.get("file_name");
            let file_size: i64 = row.get("file_size");
            let file_type: String = row.get("file_type");
            let uploaded_at_ts: i64 = row.get("uploaded_at");

            let uploaded_at = chrono::DateTime::from_timestamp(uploaded_at_ts, 0)
                .ok_or_else(|| HWSystemError::date_parse("无效的上传时间戳".to_string()))?;

            let user_id: i64 = row.get("user_id");

            Ok(Some(File {
                id,
                unique_name,
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
