use sqlx::Row;

use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::classes::requests::{CreateClassRequest, UpdateClassRequest};
use crate::models::{
    PaginationInfo, classes::entities::Class, classes::requests::ClassListQuery,
    classes::responses::ClassListResponse,
};
use crate::utils::random_code;

pub async fn create_class(storage: &SqliteStorage, class: CreateClassRequest) -> Result<Class> {
    let now = chrono::Utc::now();

    let invite_code = loop {
        let code = random_code::generate_random_code(8);
        if get_class_by_code(storage, &code).await?.is_none() {
            break code;
        }
    };

    let result = sqlx::query_as::<sqlx::Sqlite, Class>(
        "INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(class.teacher_id)
    .bind(&class.class_name)
    .bind(&class.description)
    .bind(&invite_code)
    .bind(now.timestamp()) // 使用时间戳
    .bind(now.timestamp()) // 使用时间戳
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to create class: {e}")))?;

    Ok(result)
}

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
    let result =
        sqlx::query_as::<sqlx::Sqlite, Class>("SELECT * FROM classes WHERE invite_code = ?")
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
    // 先检查用户是否存在
    if get_class_by_id(storage, class_id).await?.is_none() {
        return Ok(None);
    }

    let now = chrono::Utc::now();
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(class_name) = &update.class_name {
        updates.push("class_name = ?");
        params.push(class_name.clone());
    }

    if let Some(description) = &update.description {
        updates.push("description = ?");
        params.push(description.clone());
    }

    if updates.is_empty() {
        // 没有更新内容，直接返回当前用户
        return get_class_by_id(storage, class_id).await;
    }

    let update_at_query = format!("updated_at = {}", now.timestamp());

    updates.push(&update_at_query);

    let sql = format!(
        "UPDATE classes SET {} WHERE id = ? RETURNING *",
        updates.join(", ")
    );
    params.push(class_id.to_string());

    let mut query_builder = sqlx::query_as::<sqlx::Sqlite, Class>(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let updated_class = query_builder
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to update class: {e}")))?;

    Ok(updated_class)
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

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // 统计总数
    let count_sql = format!("SELECT COUNT(*) as total FROM classes{where_clause}");
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_row = count_query.fetch_one(&storage.pool).await.map_err(|e| {
        HWSystemError::database_operation(format!("Query class total count failed: {e}"))
    })?;
    let total: i64 = total_row.get("total");

    // 查询数据
    let data_sql = format!(
        "SELECT * 
            FROM classes{where_clause} ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );

    let mut data_query = sqlx::query_as::<sqlx::Sqlite, Class>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size).bind(offset);

    let classes = data_query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询用户列表失败: {e}")))?;

    Ok(ClassListResponse {
        items: classes,
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
