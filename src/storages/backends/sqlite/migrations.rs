use crate::errors::{HWSystemError, Result};
use sha2::{Digest, Sha256};
use sqlx::{Row, SqlitePool};
use tracing::info;

#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i32,
    pub name: String,
    pub up_sql: String,
}

pub struct SqliteMigrationManager {
    pool: SqlitePool,
}

impl SqliteMigrationManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 初始化迁移表
    pub async fn init(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at INTEGER NOT NULL,
                checksum TEXT
            )",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("创建迁移表失败: {e}")))?;

        Ok(())
    }

    /// 获取当前数据库版本
    pub async fn get_current_version(&self) -> Result<i32> {
        let result = sqlx::query("SELECT MAX(version) as version FROM schema_migrations")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询数据库版本失败: {e}")))?;

        match result {
            Some(row) => {
                let version: Option<i32> = row.get("version");
                Ok(version.unwrap_or(0))
            }
            None => Ok(0),
        }
    }

    /// 应用单个迁移
    pub async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        info!("应用迁移 v{}: {}", migration.version, migration.name);

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("开始迁移事务失败: {e}")))?;

        // 执行迁移SQL
        sqlx::query(&migration.up_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                HWSystemError::database_operation(format!(
                    "执行迁移v{}失败: {}",
                    migration.version, e
                ))
            })?;

        // 记录迁移
        let mut hasher = Sha256::new();
        hasher.update(&migration.up_sql);
        let checksum = format!("{:x}", hasher.finalize());
        sqlx::query("INSERT INTO schema_migrations (version, name, applied_at, checksum) VALUES (?, ?, ?, ?)")
            .bind(migration.version)
            .bind(&migration.name)
            .bind(chrono::Utc::now().timestamp())
            .bind(&checksum)
            .execute(&mut *tx)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("记录迁移v{}失败: {}", migration.version, e)))?;

        tx.commit().await.map_err(|e| {
            HWSystemError::database_operation(format!("提交迁移v{}失败: {}", migration.version, e))
        })?;

        info!("迁移 v{} 应用成功", migration.version);
        Ok(())
    }

    /// 运行所有待应用的迁移
    pub async fn migrate_up(&self) -> Result<()> {
        let current_version = self.get_current_version().await?;
        let migrations = get_all_migrations();

        let pending_migrations: Vec<_> = migrations
            .into_iter()
            .filter(|m| m.version > current_version)
            .collect();

        if pending_migrations.is_empty() {
            info!("数据库已是最新版本 v{}", current_version);
            return Ok(());
        }

        info!("发现 {} 个待应用的迁移", pending_migrations.len());

        for migration in pending_migrations {
            self.apply_migration(&migration).await?;
        }

        let new_version = self.get_current_version().await?;
        info!("数据库迁移完成，当前版本: v{}", new_version);

        Ok(())
    }
}

/// 获取所有迁移定义
pub fn get_all_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            name: "create_users_table_with_indexes".to_string(),
            up_sql: "
                -- 创建用户表
                CREATE TABLE users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT NOT NULL UNIQUE,
                    email TEXT NOT NULL UNIQUE,
                    password_hash TEXT NOT NULL,
                    role TEXT NOT NULL,
                    status TEXT NOT NULL,
                    profile_name TEXT,
                    student_id TEXT,
                    class TEXT,
                    avatar_url TEXT,
                    last_login INTEGER,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                -- 插入初始管理员用户 (用户名: admin, 密码: admin123)
                INSERT INTO users (username, email, password_hash, role, status, profile_name, student_id, class, avatar_url, last_login, created_at, updated_at)
                VALUES ('admin', 'admin@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'admin', 'active', 'Administrator', '000001', 'Admin', NULL, NULL, 1704067200, 1704067200);

                -- 创建索引
                CREATE INDEX idx_users_username ON users(username);
                CREATE INDEX idx_users_email ON users(email);
                CREATE INDEX idx_users_role ON users(role);
                CREATE INDEX idx_users_status ON users(status);
                CREATE INDEX idx_users_student_id ON users(student_id);
                CREATE INDEX idx_users_last_login ON users(last_login);
            ".to_string(),
        },
    ]
}
