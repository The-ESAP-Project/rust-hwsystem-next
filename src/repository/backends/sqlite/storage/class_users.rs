use sqlx::{FromRow, Row};

use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::PaginationInfo;
use crate::models::class_users::entities::{ClassUser, ClassUserRole};
use crate::models::class_users::requests::{ClassUserQuery, UpdateClassUserRequest};
use crate::models::class_users::responses::ClassUserListResponse;
use crate::models::classes::{
    entities::Class, requests::ClassListQuery, responses::ClassListResponse,
};

pub async fn join_class(
    storage: &SqliteStorage,
    user_id: i64,
    class_id: i64,
    role: ClassUserRole,
) -> Result<ClassUser> {
    let now = chrono::Utc::now().timestamp();

    // 插入关联
    sqlx::query(
        "INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
        VALUES (?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(class_id)
    .bind(user_id)
    .bind(role.to_string())
    .bind(now)
    .bind(now)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to join class: {e}")))?;

    let class_user = sqlx::query_as::<sqlx::Sqlite, ClassUser>(
        "SELECT cu.*, u.profile_name
        FROM class_users cu
        JOIN users u ON cu.user_id = u.id
        WHERE cu.class_id = ? AND cu.user_id = ?",
    )
    .bind(class_id)
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to join class: {e}")))?;

    Ok(class_user)
}

pub async fn leave_class(storage: &SqliteStorage, user_id: i64, class_id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM class_users WHERE class_id = ? AND user_id = ?")
        .bind(class_id)
        .bind(user_id)
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to leave class: {e}")))?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_class_user(
    storage: &SqliteStorage,
    class_id: i64,
    class_user_id: i64,
    update_data: UpdateClassUserRequest,
) -> Result<Option<ClassUser>> {
    // 先检查用户是否存在
    let existing_user =
        match get_class_user_by_user_id_and_class_id(storage, class_user_id, class_id).await? {
            Some(user) => user,
            None => return Ok(None),
        };

    let now = chrono::Utc::now();
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(role) = &update_data.role {
        updates.push("role = ?");
        params.push(role.to_string());
    }

    if updates.is_empty() {
        // 没有更新内容，直接返回当前班级用户
        return Ok(Some(existing_user));
    }

    let updated_at_query = format!("updated_at = {}", now.timestamp());
    updates.push(&updated_at_query);

    let sql = format!(
        "UPDATE class_users SET {} WHERE class_id = ? AND user_id = ? RETURNING *",
        updates.join(", ")
    );
    params.push(class_id.to_string());
    params.push(class_user_id.to_string());

    let mut query_builder = sqlx::query_as::<sqlx::Sqlite, ClassUser>(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let updated_class_user = query_builder
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| {
            HWSystemError::database_operation(format!("Failed to update class user: {e}"))
        })?;

    Ok(updated_class_user)
}

pub async fn list_class_users_with_pagination(
    storage: &SqliteStorage,
    class_id: i64,
    query: ClassUserQuery,
) -> Result<ClassUserListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    // 构建基本查询
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    let class_id_query = format!("cu.class_id = {class_id}");
    conditions.push(class_id_query);

    // 搜索条件
    if let Some(search) = &query.search {
        if !search.trim().is_empty() {
            conditions.push("u.profile_name LIKE ?".to_owned());
            let search_pattern = format!("%{}%", search.trim());
            params.push(search_pattern);
        }
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // 统计总数
    let count_sql = format!(
        "
    SELECT COUNT(*) as total 
    FROM class_users cu 
    JOIN users u ON cu.user_id = u.id
    {where_clause}"
    );

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
        "SELECT cu.*, u.profile_name
        FROM class_users cu
        JOIN users u ON cu.user_id = u.id
        {where_clause} ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );

    let mut data_query = sqlx::query_as::<sqlx::Sqlite, ClassUser>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size).bind(offset);

    let class_users = data_query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询班级用户列表失败: {e}")))?;

    Ok(ClassUserListResponse {
        items: class_users,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages: (total + size - 1) / size, // 向上取整
        },
    })
}

pub async fn list_user_classes_with_pagination(
    storage: &SqliteStorage,
    user_id: i64,
    query: ClassListQuery,
) -> Result<ClassListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    let total = sqlx::query_scalar::<sqlx::Sqlite, i64>(
        "SELECT COUNT(*) FROM class_users WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to count user classes: {e}")))?;

    let classes = sqlx::query_as::<sqlx::Sqlite, Class>(
        "SELECT c.* FROM classes c
        JOIN class_users cu ON cu.class_id = c.id
        WHERE cu.user_id = ?
        LIMIT ? OFFSET ?",
    )
    .bind(user_id)
    .bind(size)
    .bind(offset)
    .fetch_all(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to list user classes: {e}")))?;

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

pub async fn get_class_user_by_user_id_and_class_id(
    storage: &SqliteStorage,
    user_id: i64,
    class_id: i64,
) -> Result<Option<ClassUser>> {
    let class_user = sqlx::query_as::<sqlx::Sqlite, ClassUser>(
        "SELECT cu.*, u.profile_name
            FROM class_users cu
            JOIN users u ON cu.user_id = u.id
            WHERE cu.user_id = ? AND cu.class_id = ?",
    )
    .bind(user_id)
    .bind(class_id)
    .fetch_optional(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to get class_user: {e}")))?;

    Ok(class_user)
}

pub async fn get_class_and_class_user_by_class_id_and_code(
    storage: &SqliteStorage,
    class_id: i64,
    invite_code: &str,
    user_id: i64,
) -> Result<(Option<Class>, Option<ClassUser>)> {
    let row = sqlx::query(
        "SELECT c.*,
        cu.id as cu_id,
        cu.class_id as cu_class_id,
        cu.user_id as cu_user_id,
        cu.role as cu_role,
        cu.updated_at as cu_updated_at,
        cu.joined_at as cu_joined_at,
        u.profile_name as cu_profile_name
        FROM classes c
        LEFT JOIN class_users cu ON cu.class_id = c.id AND cu.user_id = ?
        LEFT JOIN users u ON cu.user_id = u.id
        WHERE c.id = ? AND c.invite_code = ?",
    )
    .bind(user_id)
    .bind(class_id)
    .bind(invite_code)
    .fetch_optional(&storage.pool)
    .await
    .map_err(|e| {
        HWSystemError::database_operation(format!(
            "Failed to get class and user student by id and code: {e}"
        ))
    })?;

    if let Some(row) = row {
        let class = Class::from_row(&row).map_err(|e| {
            HWSystemError::database_operation(format!("Failed to decode class: {e}"))
        })?;

        tracing::debug!("{:?}", ClassUser::from_row_prefix("cu_", &row));

        // 只要 cu_id 不为 null 就说明有成员信息
        let class_user = row
            .try_get::<i64, _>("cu_id")
            .ok()
            .and_then(|_| ClassUser::from_row_prefix("cu_", &row).ok());

        Ok((Some(class), class_user))
    } else {
        Ok((None, None))
    }
}
