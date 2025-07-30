use sqlx::Row;

use super::PostgresqlStorage;
use crate::models::{
    PaginationInfo,
    users::{
        entities::{User, UserStatus},
        requests::{CreateUserRequest, UpdateUserRequest, UserListQuery},
        responses::UserListResponse,
    },
};

use crate::errors::{HWSystemError, Result};

pub async fn create_user(storage: &PostgresqlStorage, user: CreateUserRequest) -> Result<User> {
    let now = chrono::Utc::now();

    let result = sqlx::query_as::<sqlx::Postgres, User>(
        "INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at",
    )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password) // 密码哈希应该在 Service 层完成
        .bind(user.role.to_string())
        .bind(UserStatus::Active.to_string())
        .bind(user.profile.profile_name)
        .bind(user.profile.avatar_url)
        .bind(now.timestamp()) // 使用时间戳
        .bind(now.timestamp()) // 使用时间戳
        .fetch_one(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to create user: {e}")))?;

    Ok(result)
}

pub async fn get_user_by_id(storage: &PostgresqlStorage, id: i64) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Postgres, User>(
        "SELECT id, username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at
            FROM users WHERE id = $1",
    )
        .bind(id)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Search user by ID failed: {e}")))?;

    Ok(result)
}

pub async fn get_user_by_username(
    storage: &PostgresqlStorage,
    username: &str,
) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Postgres, User>(
        "SELECT id, username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at
            FROM users WHERE username = $1",
    )
        .bind(username)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Search user by username failed: {e}")))?;

    Ok(result)
}

pub async fn get_user_by_email(storage: &PostgresqlStorage, email: &str) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Postgres, User>(
        "SELECT id, username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at
            FROM users WHERE email = $1",
    )
        .bind(email)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Search user by email failed: {e}")))?;

    Ok(result)
}

pub async fn get_user_by_username_or_email(
    storage: &PostgresqlStorage,
    identifier: &str,
) -> Result<Option<User>> {
    let result = sqlx::query_as::<sqlx::Postgres, User>(
        "SELECT id, username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at
            FROM users WHERE username = $1 OR email = $2",
    )
        .bind(identifier)
        .bind(identifier)
        .fetch_optional(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("根据用户名或邮箱查询用户失败: {e}")))?;

    Ok(result)
}

pub async fn list_users_with_pagination(
    storage: &PostgresqlStorage,
    query: UserListQuery,
) -> Result<UserListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    // Build base query
    let mut conditions = Vec::new();
    let mut params = Vec::new();
    let mut param_count = 1;

    // Search condition
    if let Some(search) = &query.search {
        if !search.trim().is_empty() {
            conditions.push(format!(
                "(username LIKE ${} OR email LIKE ${} OR profile_name LIKE ${})",
                param_count,
                param_count + 1,
                param_count + 2
            ));
            let search_pattern = format!("%{}%", search.trim());
            params.push(search_pattern.clone());
            params.push(search_pattern.clone());
            params.push(search_pattern);
            param_count += 3;
        }
    }

    // Role filter
    if let Some(role) = &query.role {
        conditions.push(format!("role = ${}", param_count));
        params.push(role.to_string());
        param_count += 1;
    }

    // Status filter
    if let Some(status) = &query.status {
        conditions.push(format!("status = ${}", param_count));
        params.push(status.to_string());
        param_count += 1;
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    // Query total count
    let count_sql = format!("SELECT COUNT(*) as total FROM users{}", where_clause);
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_row = count_query.fetch_one(&storage.pool).await.map_err(|e| {
        HWSystemError::database_operation(format!("Failed to query user total count: {e}"))
    })?;
    let total: i64 = total_row.get("total");

    // Query data
    let data_sql = format!(
        "SELECT * FROM users{} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause,
        param_count,
        param_count + 1
    );

    let mut data_query = sqlx::query_as::<sqlx::Postgres, User>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size as i64).bind(offset as i64);

    let users = data_query.fetch_all(&storage.pool).await.map_err(|e| {
        HWSystemError::database_operation(format!("Failed to query user list: {e}"))
    })?;

    let pages = (total + size as i64 - 1) / size as i64; // Ceiling division

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

pub async fn update_last_login(storage: &PostgresqlStorage, id: i64) -> Result<bool> {
    let now = chrono::Utc::now();

    let result = sqlx::query("UPDATE users SET last_login = $1 WHERE id = $2")
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
    storage: &PostgresqlStorage,
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
        updates.push(format!("email = ${}", params.len() + 1));
        params.push(email.clone());
    }

    if let Some(password) = &update.password {
        updates.push(format!("password_hash = ${}", params.len() + 1));
        params.push(password.clone());
    }

    if let Some(role) = &update.role {
        updates.push(format!("role = ${}", params.len() + 1));
        params.push(role.to_string());
    }

    if let Some(status) = &update.status {
        updates.push(format!("status = ${}", params.len() + 1));
        params.push(status.to_string());
    }

    if let Some(profile) = &update.profile {
        updates.push(format!("profile_name = ${}", params.len() + 1));
        params.push(profile.profile_name.clone());

        updates.push(format!("avatar_url = ${}", params.len() + 1));
        params.push(profile.avatar_url.clone().unwrap_or_default());
    }

    if updates.is_empty() {
        // 没有更新内容，直接返回当前用户
        return get_user_by_id(storage, id).await;
    }

    updates.push(format!("updated_at = ${}", params.len() + 1));
    params.push(now.timestamp().to_string());

    let sql = format!(
        "UPDATE users SET {} WHERE id = ${}",
        updates.join(", "),
        params.len() + 1
    );
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

pub async fn delete_user(storage: &PostgresqlStorage, id: i64) -> Result<bool> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to delete user: {e}")))?;

    Ok(result.rows_affected() > 0)
}
