use super::SqliteStorage;
use crate::models::files::entities::File;

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

    let result = sqlx::query_as::<sqlx::Sqlite, File>(
        "INSERT INTO files (submission_token, file_name, file_size, file_type, uploaded_at, user_id)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING submission_token, file_name, file_size, file_type, uploaded_at, user_id",
    )
    .bind(submission_token)
    .bind(file_name)
    .bind(file_size)
    .bind(file_type)
    .bind(now.timestamp())
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("上传文件失败: {e}")))?;

    Ok(result)
}

pub async fn get_file_by_token(storage: &SqliteStorage, file_id: String) -> Result<Option<File>> {
    let result =
        sqlx::query_as::<sqlx::Sqlite, File>("SELECT * FROM files WHERE submission_token = ?")
            .bind(file_id)
            .fetch_optional(&storage.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询文件失败: {e}")))?;

    match result {
        Some(row) => Ok(Some(row)),
        None => Ok(None),
    }
}
