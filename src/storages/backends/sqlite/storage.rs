use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sqlx::{Row, SqlitePool, sqlite, sqlite::SqliteConnectOptions};
use std::env;
use tracing::warn;

use super::migrations::SqliteMigrationManager;
use crate::api_models::{
    PaginationInfo,
    users::{
        entities::{User, UserProfile, UserRole, UserStatus},
        requests::{CreateUserRequest, UpdateUserRequest, UserListQuery},
        responses::UserListResponse,
    },
};
use crate::errors::{HWSystemError, Result};
use crate::storages::Storage;
use async_trait::async_trait;

// 注册 SQLite 存储插件
declare_storage_plugin!("sqlite", SqliteStorage);

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new_async() -> Result<Self> {
        let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| "hwsystem.db".into());

        // 创建连接池
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&db_path)
                .create_if_missing(true)
                .journal_mode(sqlite::SqliteJournalMode::Wal)
                .synchronous(sqlite::SqliteSynchronous::Normal)
                .busy_timeout(std::time::Duration::from_secs(5))
                .pragma("cache_size", "-64000")
                .pragma("temp_store", "memory")
                .pragma("mmap_size", "536870912")
                .pragma("wal_autocheckpoint", "1000"),
        )
        .await
        .map_err(|e| HWSystemError::database_connection(format!("无法连接到数据库: {e}")))?;

        let storage = SqliteStorage { pool };

        // 初始化迁移系统并运行迁移
        let migration_manager = SqliteMigrationManager::new(storage.pool.clone());
        migration_manager.init().await?;
        migration_manager.migrate_up().await?;

        warn!("SqliteStorage initialized, database path: {}", db_path);

        Ok(storage)
    }

    fn user_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<User> {
        let id: i64 = row.get("id");
        let username: String = row.get("username");
        let role_str: String = row.get("role");
        let email: String = row.get("email");
        let status_str: String = row.get("status");
        let created_at_ts: i64 = row.get("created_at");
        let updated_at_ts: i64 = row.get("updated_at");

        // 获取可选字段
        let profile_name: Option<String> = row.try_get("profile_name").ok();
        let student_id: Option<String> = row.try_get("student_id").ok();
        let class: Option<String> = row.try_get("class").ok();
        let avatar_url: Option<String> = row.try_get("avatar_url").ok();
        let last_login_ts: Option<i64> = row.try_get("last_login").ok();

        let role = role_str
            .parse::<UserRole>()
            .map_err(|e| HWSystemError::validation(format!("角色解析失败: {e}")))?;
        let status = status_str
            .parse::<UserStatus>()
            .map_err(|e| HWSystemError::validation(format!("状态解析失败: {e}")))?;

        // 从时间戳转换为DateTime
        let created_at = chrono::DateTime::from_timestamp(created_at_ts, 0)
            .ok_or_else(|| HWSystemError::date_parse("无效的创建时间时间戳".to_string()))?;

        let updated_at = chrono::DateTime::from_timestamp(updated_at_ts, 0)
            .ok_or_else(|| HWSystemError::date_parse("无效的更新时间时间戳".to_string()))?;

        let last_login = if let Some(last_login_ts) = last_login_ts {
            Some(
                chrono::DateTime::from_timestamp(last_login_ts, 0).ok_or_else(|| {
                    HWSystemError::date_parse("无效的最后登录时间时间戳".to_string())
                })?,
            )
        } else {
            None
        };

        let profile = if profile_name.is_some()
            || student_id.is_some()
            || class.is_some()
            || avatar_url.is_some()
        {
            Some(UserProfile {
                name: profile_name.unwrap_or_default(),
                student_id,
                class,
                avatar_url,
            })
        } else {
            None
        };

        Ok(User {
            id,
            username,
            email,
            role,
            status,
            profile,
            last_login,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn create_user(&self, user: CreateUserRequest) -> Result<User> {
        let now = chrono::Utc::now();

        // 使用 Argon2 进行密码哈希
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|e| HWSystemError::validation(format!("密码哈希失败: {e}")))?
            .to_string();

        let result = sqlx::query(
            "INSERT INTO users (username, email, password_hash, role, status, profile_name, student_id, class, avatar_url, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(user.role.to_string())
        .bind(UserStatus::Active.to_string())
        .bind(user.profile.as_ref().map(|p| &p.name))
        .bind(user.profile.as_ref().and_then(|p| p.student_id.as_deref()))
        .bind(user.profile.as_ref().and_then(|p| p.class.as_deref()))
        .bind(user.profile.as_ref().and_then(|p| p.avatar_url.as_deref()))
        .bind(now.timestamp()) // 使用时间戳
        .bind(now.timestamp()) // 使用时间戳
        .fetch_one(&self.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("创建用户失败: {e}")))?;

        let id: i64 = result.get("id");

        Ok(User {
            id,
            username: user.username,
            email: user.email,
            role: user.role,
            status: UserStatus::Active,
            profile: user.profile,
            last_login: None,
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        let result = sqlx::query(
            "SELECT id, username, email, role, status, profile_name, student_id, class, avatar_url, last_login, created_at, updated_at 
             FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询用户失败: {e}")))?;

        match result {
            Some(row) => Ok(Some(Self::user_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let result = sqlx::query(
            "SELECT id, username, email, role, status, profile_name, student_id, class, avatar_url, last_login, created_at, updated_at 
             FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("根据用户名查询用户失败: {e}")))?;

        match result {
            Some(row) => Ok(Some(Self::user_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_users_with_pagination(&self, query: UserListQuery) -> Result<UserListResponse> {
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
            .fetch_one(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询用户总数失败: {e}")))?;
        let total: i64 = total_row.get("total");

        // 查询数据
        let data_sql = format!(
            "SELECT id, username, email, role, status, profile_name, student_id, class, avatar_url, last_login, created_at, updated_at 
             FROM users{where_clause} ORDER BY created_at DESC LIMIT ? OFFSET ?"
        );

        let mut data_query = sqlx::query(&data_sql);
        for param in &params {
            data_query = data_query.bind(param);
        }
        data_query = data_query.bind(size).bind(offset);

        let rows = data_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询用户列表失败: {e}")))?;

        let mut users = Vec::new();
        for row in rows {
            users.push(Self::user_from_row(&row)?);
        }

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

    async fn update_last_login(&self, id: i64) -> Result<bool> {
        let now = chrono::Utc::now();

        let result = sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("更新最后登录时间失败: {e}")))?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>> {
        // 先检查用户是否存在
        if self.get_user_by_id(id).await?.is_none() {
            return Ok(None);
        }

        let now = chrono::Utc::now();
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(email) = &update.email {
            updates.push("email = ?");
            params.push(email.clone());
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
            updates.push("student_id = ?");
            updates.push("class = ?");
            updates.push("avatar_url = ?");
            params.push(profile.name.clone());
            params.push(profile.student_id.clone().unwrap_or_default());
            params.push(profile.class.clone().unwrap_or_default());
            params.push(profile.avatar_url.clone().unwrap_or_default());
        }

        if updates.is_empty() {
            // 没有更新内容，直接返回当前用户
            return self.get_user_by_id(id).await;
        }

        updates.push("updated_at = ?");
        params.push(now.to_rfc3339());

        let sql = format!("UPDATE users SET {} WHERE id = ?", updates.join(", "));
        params.push(id.to_string());

        let mut query_builder = sqlx::query(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder
            .execute(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("更新用户失败: {e}")))?;

        self.get_user_by_id(id).await
    }

    async fn delete_user(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("删除用户失败: {e}")))?;

        Ok(result.rows_affected() > 0)
    }
}
