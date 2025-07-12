use sqlx::Row;

use super::SqliteStorage;
use crate::models::{
    PaginationInfo,
    users::{
        entities::{User, UserStatus},
        requests::{CreateUserRequest, UpdateUserRequest, UserListQuery},
        responses::UserListResponse,
    },
};

use crate::errors::{HWSystemError, Result};

pub async fn create_user(storage: &SqliteStorage, user: CreateUserRequest) -> Result<User> {
    let now = chrono::Utc::now();

    let result = sqlx::query_as::<sqlx::Sqlite, User>(
        "INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, created_at, updated_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password) // 密码哈希应该在 Service 层完成
    .bind(user.role.to_string())
    .bind(UserStatus::Active.to_string())
    .bind(user.profile.as_ref().map(|p| &p.profile_name))
    .bind(user.profile.as_ref().and_then(|p| p.avatar_url.as_deref()))
    .bind(now.timestamp()) // 使用时间戳
    .bind(now.timestamp()) // 使用时间戳
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to create user: {e}")))?;

    Ok(result)
}

pub async fn get_user_by_id(storage: &SqliteStorage, id: i64) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Sqlite, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Search user by ID failed: {e}")))?;

    match result {
        Some(row) => Ok(Some(row)),
        None => Ok(None),
    }
}

pub async fn get_user_by_username(storage: &SqliteStorage, username: &str) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Sqlite, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| {
            HWSystemError::database_operation(format!("Search user by username failed: {e}"))
        })?;

    match result {
        Some(row) => Ok(Some(row)),
        None => Ok(None),
    }
}

pub async fn get_user_by_email(storage: &SqliteStorage, email: &str) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Sqlite, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| {
            HWSystemError::database_operation(format!("Search user by email failed: {e}"))
        })?;

    match result {
        Some(row) => Ok(Some(row)),
        None => Ok(None),
    }
}

pub async fn get_user_by_username_or_email(
    storage: &SqliteStorage,
    identifier: &str,
) -> Result<Option<User>> {
    let result =
        sqlx::query_as::<sqlx::Sqlite, User>("SELECT * FROM users WHERE username = ? OR email = ?")
            .bind(identifier)
            .bind(identifier)
            .fetch_optional(&storage.pool)
            .await
            .map_err(|e| {
                HWSystemError::database_operation(format!("根据用户名或邮箱查询用户失败: {e}"))
            })?;

    match result {
        Some(row) => Ok(Some(row)),
        None => Ok(None),
    }
}

pub async fn list_users_with_pagination(
    storage: &SqliteStorage,
    query: UserListQuery,
) -> Result<UserListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    // 构建基本查询
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    // 搜索条件
    if let Some(search) = &query.search {
        if !search.trim().is_empty() {
            conditions.push("(username LIKE ? OR email LIKE ? OR profile_name LIKE ?)");
            let search_pattern = format!("%{}%", search.trim());
            params.push(search_pattern.clone());
            params.push(search_pattern.clone());
            params.push(search_pattern);
        }
    }

    // 角色筛选
    if let Some(role) = &query.role {
        conditions.push("role = ?");
        params.push(role.to_string());
    }

    // 状态筛选
    if let Some(status) = &query.status {
        conditions.push("status = ?");
        params.push(status.to_string());
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // 查询总数
    let count_sql = format!("SELECT COUNT(*) as total FROM users{where_clause}");
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_row = count_query
        .fetch_one(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询用户总数失败: {e}")))?;
    let total: i64 = total_row.get("total");

    // 查询数据
    let data_sql = format!(
        "SELECT * 
            FROM users{where_clause} ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );

    let mut data_query = sqlx::query_as::<sqlx::Sqlite, User>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size).bind(offset);

    let users = data_query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询用户列表失败: {e}")))?;

    let pages = (total + size - 1) / size; // 向上取整

    Ok(UserListResponse {
        items: users,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages,
        },
    })
}

pub async fn update_last_login(storage: &SqliteStorage, id: i64) -> Result<bool> {
    let now = chrono::Utc::now();

    let result = sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
        .bind(now.timestamp())
        .bind(id)
        .execute(&storage.pool)
        .await
        .map_err(|e| {
            HWSystemError::database_operation(format!("Failed to update last login time: {e}"))
        })?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_user(
    storage: &SqliteStorage,
    id: i64,
    update: UpdateUserRequest,
) -> Result<Option<User>> {
    // 先检查用户是否存在
    if get_user_by_id(storage, id).await?.is_none() {
        return Ok(None);
    }

    let now = chrono::Utc::now();
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(email) = &update.email {
        updates.push("email = ?");
        params.push(email.clone());
    }

    if let Some(password) = &update.password {
        updates.push("password_hash = ?");
        params.push(password.clone());
    }

    if let Some(role) = &update.role {
        updates.push("role = ?");
        params.push(role.to_string());
    }

    if let Some(status) = &update.status {
        updates.push("status = ?");
        params.push(status.to_string());
    }

    if let Some(profile) = &update.profile {
        updates.push("profile_name = ?");
        updates.push("avatar_url = ?");
        params.push(profile.profile_name.clone());
        params.push(profile.avatar_url.clone().unwrap_or_default());
    }

    if updates.is_empty() {
        // 没有更新内容，直接返回当前用户
        return get_user_by_id(storage, id).await;
    }

    let update_at_query = format!("updated_at = {}", now.timestamp());

    updates.push(&update_at_query);

    let sql = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));
    params.push(id.to_string());

    let mut query_builder = sqlx::query(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to update user: {e}")))?;

    get_user_by_id(storage, id).await
}

pub async fn delete_user(storage: &SqliteStorage, id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to delete user: {e}")))?;

    Ok(result.rows_affected() > 0)
}
