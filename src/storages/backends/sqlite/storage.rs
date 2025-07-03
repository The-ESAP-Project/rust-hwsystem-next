use sqlx::{Row, SqlitePool, sqlite, sqlite::SqliteConnectOptions};
use std::env;
use tracing::warn;

use super::migrations::SqliteMigrationManager;
use crate::errors::{HWSystemError, Result};
use crate::storages::{CreateUserRequest, Storage, UpdateUserRequest, User, UserRole, UserStatus};
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

    /// 获取迁移管理器
    pub fn get_migration_manager(&self) -> SqliteMigrationManager {
        SqliteMigrationManager::new(self.pool.clone())
    }

    fn user_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<User> {
        let id: i64 = row.get("id");
        let username: String = row.get("username");
        let role_str: String = row.get("role");
        let email: String = row.get("email");
        let status_str: String = row.get("status");
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        let role = role_str
            .parse::<UserRole>()
            .map_err(|e| HWSystemError::validation(format!("角色解析失败: {e}")))?;
        let status = status_str
            .parse::<UserStatus>()
            .map_err(|e| HWSystemError::validation(format!("状态解析失败: {e}")))?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| HWSystemError::date_parse(format!("创建时间解析失败: {e}")))?
            .with_timezone(&chrono::Utc);

        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|e| HWSystemError::date_parse(format!("更新时间解析失败: {e}")))?
            .with_timezone(&chrono::Utc);

        Ok(User {
            id,
            username,
            role,
            email,
            status,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn create_user(&self, user: CreateUserRequest) -> Result<User> {
        let now = chrono::Utc::now();
        let status = user.status.unwrap_or(UserStatus::Active);

        let result = sqlx::query(
            "INSERT INTO users (username, role, email, status, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        )
        .bind(&user.username)
        .bind(user.role.to_string())
        .bind(&user.email)
        .bind(status.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("创建用户失败: {e}")))?;

        let id: i64 = result.get("id");

        Ok(User {
            id,
            username: user.username,
            role: user.role,
            email: user.email,
            status,
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        let result = sqlx::query(
            "SELECT id, username, role, email, status, created_at, updated_at 
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

    async fn search_users(
        &self,
        query: &str,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>> {
        let mut sql = "SELECT id, username, role, email, status, created_at, updated_at 
                       FROM users WHERE (username LIKE ? OR email LIKE ?)"
            .to_string();
        let mut params = vec![format!("%{}%", query), format!("%{}%", query)];

        if let Some(role) = role {
            sql.push_str(" AND role = ?");
            params.push(role.to_string());
        }

        if let Some(status) = status {
            sql.push_str(" AND status = ?");
            params.push(status.to_string());
        }

        sql.push_str(" ORDER BY created_at DESC");

        let mut query_builder = sqlx::query(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("搜索用户失败: {e}")))?;

        let mut users = Vec::new();
        for row in rows {
            users.push(Self::user_from_row(&row)?);
        }

        Ok(users)
    }

    async fn list_users(
        &self,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>> {
        let mut sql = "SELECT id, username, role, email, status, created_at, updated_at 
                       FROM users WHERE 1=1"
            .to_string();
        let mut params = Vec::new();

        if let Some(role) = role {
            sql.push_str(" AND role = ?");
            params.push(role.to_string());
        }

        if let Some(status) = status {
            sql.push_str(" AND status = ?");
            params.push(status.to_string());
        }

        sql.push_str(" ORDER BY created_at DESC");

        let mut query_builder = sqlx::query(&sql);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询用户列表失败: {e}")))?;

        let mut users = Vec::new();
        for row in rows {
            users.push(Self::user_from_row(&row)?);
        }

        Ok(users)
    }

    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>> {
        // 先检查用户是否存在
        if self.get_user_by_id(id).await?.is_none() {
            return Ok(None);
        }

        let now = chrono::Utc::now();
        let mut updates = Vec::new();
        let mut params = Vec::new();

        if let Some(username) = &update.username {
            updates.push("username = ?");
            params.push(username.clone());
        }

        if let Some(role) = &update.role {
            updates.push("role = ?");
            params.push(role.to_string());
        }

        if let Some(email) = &update.email {
            updates.push("email = ?");
            params.push(email.clone());
        }

        if let Some(status) = &update.status {
            updates.push("status = ?");
            params.push(status.to_string());
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

    async fn run_migrations(&self) -> Result<()> {
        let migration_manager = self.get_migration_manager();
        migration_manager.migrate_up().await
    }

    async fn get_schema_version(&self) -> Result<i32> {
        let migration_manager = self.get_migration_manager();
        migration_manager.get_current_version().await
    }
}
