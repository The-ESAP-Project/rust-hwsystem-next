use sqlx::Row;

use super::PostgresqlStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::class_users::entities::ClassUserRole;
use crate::models::classes::requests::{CreateClassRequest, UpdateClassRequest};
use crate::models::{
    PaginationInfo, classes::entities::Class, classes::requests::ClassListQuery,
    classes::responses::ClassListResponse,
};
use crate::utils::random_code;

pub async fn create_class(storage: &PostgresqlStorage, class: CreateClassRequest) -> Result<Class> {
    let now = chrono::Utc::now();

    let invite_code = loop {
        let code = random_code::generate_random_code(8);
        if get_class_by_code(storage, &code).await?.is_none() {
            break code;
        }
    };

    let result = sqlx::query_as::<sqlx::Postgres, Class>(
        "INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
        .bind(class.teacher_id)
        .bind(&class.class_name)
        .bind(&class.description)
        .bind(&invite_code)
        .bind(now.timestamp())
        .bind(now.timestamp())
        .fetch_one(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to create class: {e}")))?;

    super::class_users::join_class(storage, class.teacher_id, result.id, ClassUserRole::Teacher)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to join class: {e}")))?;

    Ok(result)
}

pub async fn get_class_by_id(storage: &PostgresqlStorage, class_id: i64) -> Result<Option<Class>> {
    let result = sqlx::query_as::<sqlx::Postgres, Class>("SELECT * FROM classes WHERE id = $1")
        .bind(class_id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn get_class_by_name(storage: &PostgresqlStorage, class_name: &str) -> Result<Option<Class>> {
    let result = sqlx::query_as::<sqlx::Postgres, Class>("SELECT * FROM classes WHERE class_name = $1")
        .bind(class_name)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn get_class_by_code(storage: &PostgresqlStorage, class_code: &str) -> Result<Option<Class>> {
    let result =
        sqlx::query_as::<sqlx::Postgres, Class>("SELECT * FROM classes WHERE invite_code = $1")
            .bind(class_code)
            .fetch_optional(&storage.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("Query class failed: {e}")))?;

    Ok(result)
}

pub async fn update_class(
    storage: &PostgresqlStorage,
    class_id: i64,
    update: UpdateClassRequest,
) -> Result<Option<Class>> {
    // Check if class exists
    if get_class_by_id(storage, class_id).await?.is_none() {
        return Ok(None);
    }

    let now = chrono::Utc::now();
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(class_name) = &update.class_name {
        updates.push(format!("class_name = ${}", param_count));
        params.push(class_name.clone());
        param_count += 1;
    }

    if let Some(description) = &update.description {
        updates.push(format!("description = ${}", param_count));
        params.push(description.clone());
        param_count += 1;
    }

    if updates.is_empty() {
        // No updates, return current class
        return get_class_by_id(storage, class_id).await;
    }

    updates.push(format!("updated_at = {}", now.timestamp()));

    let sql = format!(
        "UPDATE classes SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );
    params.push(class_id.to_string());

    let mut query_builder = sqlx::query_as::<sqlx::Postgres, Class>(&sql);
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
    storage: &PostgresqlStorage,
    query: ClassListQuery,
) -> Result<ClassListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    // Build base query
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    let mut param_count = 1;

    // Teacher ID search
    if let Some(teacher_id) = &query.teacher_id {
        conditions.push(format!("teacher_id = ${}", param_count));
        params.push(teacher_id.to_string());
        param_count += 1;
    }

    // Search condition
    if let Some(search) = &query.search {
        if !search.trim().is_empty() {
            conditions.push(format!("(class_name LIKE ${} OR description LIKE ${})", param_count, param_count + 1));
            let search_pattern = format!("%{}%", search.trim());
            params.push(search_pattern.clone());
            params.push(search_pattern);
            param_count += 2;
        }
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Count total
    let count_sql = format!("SELECT COUNT(*) as total FROM classes{}", where_clause);
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_row = count_query.fetch_one(&storage.pool).await.map_err(|e| {
        HWSystemError::database_operation(format!("Query class total count failed: {e}"))
    })?;
    let total: i64 = total_row.get("total");

    // Query data
    let data_sql = format!(
        "SELECT * FROM classes{} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause,
        param_count,
        param_count + 1
    );

    let mut data_query = sqlx::query_as::<sqlx::Postgres, Class>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size as i64).bind(offset as i64);

    let classes = data_query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to query class list: {e}")))?;

    Ok(ClassListResponse {
        items: classes,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages: (total + size as i64 - 1) / size as i64, // Ceiling division
        },
    })
}

pub async fn delete_class(storage: &PostgresqlStorage, class_id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM classes WHERE id = $1")
        .bind(class_id)
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to delete class: {e}")))?;

    Ok(result.rows_affected() > 0)
}