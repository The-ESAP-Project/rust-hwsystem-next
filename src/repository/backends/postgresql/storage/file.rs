use super::PostgresqlStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::files::entities::File;

pub async fn upload_file(
    storage: &PostgresqlStorage,
    submission_token: &str,
    file_name: &str,
    file_size: &i64,
    file_type: &str,
    user_id: i64,
) -> Result<File> {
    let now = chrono::Utc::now().naive_utc();

    let result = sqlx::query_as::<_, File>(
        "INSERT INTO files (submission_token, file_name, file_size, file_type, uploaded_at, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING submission_token, file_name, file_size, file_type, uploaded_at, user_id",
    )
    .bind(submission_token)
    .bind(file_name)
    .bind(file_size)
    .bind(file_type)
    .bind(now)
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("上传文件失败: {e}")))?;

    Ok(result)
}

pub async fn get_file_by_token(storage: &PostgresqlStorage, file_id: &str) -> Result<Option<File>> {
    let result = sqlx::query_as::<_, File>("SELECT * FROM files WHERE submission_token = $1")
        .bind(file_id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询文件失败: {e}")))?;

    Ok(result)
}
