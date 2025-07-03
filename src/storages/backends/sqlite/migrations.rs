use crate::errors::{HWSystemError, Result};
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
                applied_at TEXT NOT NULL,
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
        let checksum = format!("{:x}", md5::compute(&migration.up_sql));
        sqlx::query("INSERT INTO schema_migrations (version, name, applied_at, checksum) VALUES (?, ?, ?, ?)")
            .bind(migration.version)
            .bind(&migration.name)
            .bind(chrono::Utc::now().to_rfc3339())
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
            name: "create_users_table".to_string(),
            up_sql: "CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                role TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"
            .to_string(),
        },
        Migration {
            version: 2,
            name: "create_users_username_index".to_string(),
            up_sql: "CREATE INDEX idx_users_username ON users(username)".to_string(),
        },
        Migration {
            version: 3,
            name: "create_users_email_index".to_string(),
            up_sql: "CREATE INDEX idx_users_email ON users(email)".to_string(),
        },
        Migration {
            version: 4,
            name: "create_users_role_index".to_string(),
            up_sql: "CREATE INDEX idx_users_role ON users(role)".to_string(),
        },
    ]
}
