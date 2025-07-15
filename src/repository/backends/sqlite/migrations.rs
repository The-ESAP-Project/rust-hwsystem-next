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
                    FOREIGN KEY (teacher_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- 创建班级学生关联表
                CREATE TABLE class_users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    class_id INTEGER NOT NULL,
                    user_id INTEGER NOT NULL,
                    role TEXT NOT NULL,
                    updated_at INTEGER NOT NULL,
                    joined_at INTEGER NOT NULL,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- 创建作业表
                CREATE TABLE homeworks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    class_id INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    content TEXT,
                    attachments BLOB,  -- file_submission_token JSONB
                    max_score REAL NOT NULL,
                    deadline INTEGER,
                    allow_late_submission BOOLEAN NOT NULL DEFAULT 0,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
                );

                -- 创建提交表
                CREATE TABLE submissions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    homework_id INTEGER NOT NULL,
                    creator_id INTEGER NOT NULL,
                    content TEXT NOT NULL,
                    attachments BLOB,  -- file_submission_token JSONB
                    submitted_at INTEGER NOT NULL,
                    FOREIGN KEY (homework_id) REFERENCES homeworks(id) ON DELETE CASCADE,
                    FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- 创建评分表
                CREATE TABLE grades (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    submission_id INTEGER NOT NULL,
                    grader_id INTEGER NOT NULL,      -- 评分人
                    score REAL NOT NULL,             -- 分数
                    comment TEXT,                    -- 评语
                    graded_at INTEGER NOT NULL,      -- 评分时间
                    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE,
                    FOREIGN KEY (grader_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- 创建文件关联表
                CREATE TABLE files (
                    submission_token TEXT PRIMARY KEY,
                    file_name TEXT NOT NULL,
                    file_size INTEGER NOT NULL,
                    file_type TEXT NOT NULL,
                    uploaded_at INTEGER NOT NULL,
                    citation_count INTEGER DEFAULT 0,
                    user_id INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- 插入初始管理员用户 (用户名: admin, 密码: admin123)
                -- 到时候要更改为随机密码
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
                CREATE INDEX idx_class_users_class_id ON class_users(class_id);
                CREATE INDEX idx_class_users_user_id ON class_users(user_id);
                CREATE INDEX idx_class_users_role ON class_users(role);

                -- 创建文件关联表索引
                CREATE INDEX idx_files_citation_count ON files(citation_count);
                CREATE INDEX idx_files_user_id ON files(user_id);
            ".to_string(),
        },
        Migration {
            version: 2,
            name: "add_test_data".to_string(),
            up_sql: "
                -- 插入测试数据
                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_1', 'test@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'teacher', 'active', 'Test User', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_2', 'test2@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'teacher', 'active', 'Test User 2', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_3', 'test3@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 3', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_4', 'test4@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 4', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_5', 'test5@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 5', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_6', 'test6@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 6', NULL, NULL, 1704067200, 1704067200);

                INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at)
                VALUES (2, 'Test Class 1', 'This is a test class', 'TEST123', 1704067200, 1704067200);

                INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at)
                VALUES (3, 'Test Class 2', 'This is another test class', 'TEST456', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 2, 'teacher', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 4, 'class_representative', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 5, 'student', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 3, 'teacher', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 6, 'class_representative', 1704067200, 1704067200);

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 7, 'student', 1704067200, 1704067200);

                -- 正式环境需要删除这些测试数据
                ".to_string(),
        },
    ]
}
