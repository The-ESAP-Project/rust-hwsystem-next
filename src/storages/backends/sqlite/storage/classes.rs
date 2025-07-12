use sqlx::Row;

use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::classes::requests::UpdateClassRequest;
use crate::models::{
    PaginationInfo, classes::entities::Class, classes::requests::ClassListQuery,
    classes::responses::ClassListResponse,
};

pub async fn get_class_by_id(storage: &SqliteStorage, class_id: i64) -> Result<Option<Class>> {
    let result = sqlx::query_as::<sqlx::Sqlite, Class>("SELECT * FROM classes WHERE id = ?")
        .bind(class_id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn get_class_by_name(storage: &SqliteStorage, class_name: &str) -> Result<Option<Class>> {
    let result = sqlx::query_as::<sqlx::Sqlite, Class>("SELECT * FROM classes WHERE name = ?")
        .bind(class_name)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn get_class_by_code(storage: &SqliteStorage, class_code: &str) -> Result<Option<Class>> {
    let result = sqlx::query_as::<sqlx::Sqlite, Class>("SELECT * FROM classes WHERE code = ?")
        .bind(class_code)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn update_class(
    storage: &SqliteStorage,
    class_id: i64,
    update: UpdateClassRequest,
) -> Result<Option<Class>> {
    unimplemented!("Update class functionality is not implemented yet");
}

pub async fn list_classes_with_pagination(
    storage: &SqliteStorage,
    query: ClassListQuery,
) -> Result<ClassListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    // 构建基本查询
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    // 搜索条件
    if let Some(search) = &query.search {
        if !search.trim().is_empty() {
            conditions.push("(class_name LIKE ? OR description LIKE ?)");
            let search_pattern = format!("%{}%", search.trim());
            params.push(search_pattern.clone());
            params.push(search_pattern);
        }
    }

    // 统计总数
    let count_sql = "SELECT COUNT(*) as total FROM classes".to_string();
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_row = count_query.fetch_one(&storage.pool).await.map_err(|e| {
        HWSystemError::database_operation(format!("Query class total count failed: {e}"))
    })?;
    let total: i64 = total_row.get("total");

    let result = sqlx::query_as::<sqlx::Sqlite, Class>(
        "SELECT * FROM classes ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(size)
    .bind(offset)
    .fetch_all(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(ClassListResponse {
        items: result,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages: (total + size - 1) / size, // 向上取整
        },
    })
}

pub async fn delete_class(storage: &SqliteStorage, class_id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM classes WHERE id = ?")
        .bind(class_id)
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to delete class: {e}")))?;

    Ok(result.rows_affected() > 0)
}
