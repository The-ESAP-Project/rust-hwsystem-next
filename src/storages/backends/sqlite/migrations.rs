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
            name: "create_table_with_indexes".to_string(),
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
                    avatar_url TEXT,
                    last_login INTEGER,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                );

                -- 创建班级表
                CREATE TABLE classes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    teacher_id INTEGER NOT NULL,
                    class_name TEXT NOT NULL UNIQUE,
                    description TEXT,
                    invite_code TEXT NOT NULL UNIQUE,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY (teacher_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- 创建班级学生关联表
                CREATE TABLE class_students (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    class_id INTEGER NOT NULL,
                    student_id INTEGER NOT NULL,
                    role TEXT NOT NULL,
                    joined_at INTEGER NOT NULL,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                    FOREIGN KEY (student_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- 创建作业表
                CREATE TABLE assignments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id INTEGER NOT NULL,
                    class_id INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT,
                    due_date INTEGER,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE 
                );

                -- 创建提交表
                CREATE TABLE submissions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    assignment_id INTEGER NOT NULL,
                    creator_id INTEGER NOT NULL,
                    content TEXT NOT NULL,
                    submitted_at INTEGER NOT NULL,
                    FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE CASCADE,
                    FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- 创建文件表
                CREATE TABLE files (
                    submission_token TEXT PRIMARY KEY,
                    file_name TEXT NOT NULL,
                    file_size INTEGER NOT NULL,
                    file_type TEXT NOT NULL,
                    uploaded_at INTEGER NOT NULL,
                    user_id INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- 插入初始管理员用户 (用户名: admin, 密码: admin123)
                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('admin', 'admin@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'admin', 'active', 'Administrator', NULL, NULL, 1704067200, 1704067200);

                -- 用户表索引
                CREATE INDEX idx_users_username ON users(username);
                CREATE INDEX idx_users_email ON users(email);
                CREATE INDEX idx_users_role ON users(role);
                CREATE INDEX idx_users_status ON users(status);
                CREATE INDEX idx_users_last_login ON users(last_login);

                -- 班级表索引
                CREATE INDEX idx_classes_class_name ON classes(class_name);
                CREATE INDEX idx_classes_teacher_id ON classes(teacher_id);
                CREATE INDEX idx_classes_invite_code ON classes(invite_code);

                -- 班级学生关联表索引
            ".to_string(),
        },
    ]
}
