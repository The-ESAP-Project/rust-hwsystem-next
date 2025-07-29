use crate::errors::{HWSystemError, Result};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use tracing::info;

#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i32,
    pub name: String,
    pub up_sql: String,
}

pub struct PostgresqlMigrationManager {
    pool: PgPool,
}

impl PostgresqlMigrationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize migration table
    pub async fn init(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at BIGINT NOT NULL,
                checksum TEXT
            )
            "#,
        )
            .execute(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("Failed to create migrations table: {e}")))?;

        Ok(())
    }

    /// Get current database version
    pub async fn get_current_version(&self) -> Result<i32> {
        let result = sqlx::query("SELECT MAX(version) as version FROM schema_migrations")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("Failed to query database version: {e}")))?;

        match result {
            Some(row) => {
                let version: Option<i32> = row.get("version");
                Ok(version.unwrap_or(0))
            }
            None => Ok(0),
        }
    }

    /// Apply a single migration
    pub async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        info!("Applying migration v{}: {}", migration.version, migration.name);

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("Failed to begin migration transaction: {e}")))?;

        // Execute each statement separately (PostgreSQL requires this for some DDL statements)
        for stmt in migration.up_sql.split(';') {
            let trimmed = stmt.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| {
                        HWSystemError::database_operation(format!(
                            "Failed to execute migration v{}: {}",
                            migration.version, e
                        ))
                    })?;
            }
        }

        // Record the migration
        let mut hasher = Sha256::new();
        hasher.update(&migration.up_sql);
        let checksum = format!("{:x}", hasher.finalize());

        sqlx::query(
            "INSERT INTO schema_migrations (version, name, applied_at, checksum) VALUES ($1, $2, $3, $4)",
        )
            .bind(migration.version)
            .bind(&migration.name)
            .bind(chrono::Utc::now().timestamp())
            .bind(&checksum)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                HWSystemError::database_operation(format!(
                    "Failed to record migration v{}: {}",
                    migration.version, e
                ))
            })?;

        tx.commit()
            .await
            .map_err(|e| {
                HWSystemError::database_operation(format!(
                    "Failed to commit migration v{}: {}",
                    migration.version, e
                ))
            })?;

        info!("Successfully applied migration v{}", migration.version);
        Ok(())
    }

    /// Run all pending migrations
    pub async fn migrate_up(&self) -> Result<()> {
        let current_version = self.get_current_version().await?;
        let migrations = get_all_migrations();

        let pending_migrations: Vec<_> = migrations
            .into_iter()
            .filter(|m| m.version > current_version)
            .collect();

        if pending_migrations.is_empty() {
            info!("Database is already at latest version v{}", current_version);
            return Ok(());
        }

        info!("Found {} migrations to apply", pending_migrations.len());

        for migration in pending_migrations {
            self.apply_migration(&migration).await?;
        }

        let new_version = self.get_current_version().await?;
        info!("Database migrations complete, current version: v{}", new_version);

        Ok(())
    }
}

/// Get all migration definitions
pub fn get_all_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            name: "create_table_with_indexes".to_string(),
            up_sql: r#"
                -- Create users table
                CREATE TABLE users (
                    id SERIAL PRIMARY KEY,
                    username TEXT NOT NULL UNIQUE,
                    email TEXT NOT NULL UNIQUE,
                    password_hash TEXT NOT NULL,
                    role TEXT NOT NULL,
                    status TEXT NOT NULL,
                    profile_name TEXT,
                    avatar_url TEXT,
                    last_login TIMESTAMP,
                    created_at TIMESTAMP NOT NULL,
                    updated_at TIMESTAMP NOT NULL
                );

                -- Create classes table
                CREATE TABLE classes (
                    id SERIAL PRIMARY KEY,
                    teacher_id INTEGER NOT NULL,
                    class_name TEXT NOT NULL UNIQUE,
                    description TEXT,
                    invite_code TEXT NOT NULL UNIQUE,
                    created_at TIMESTAMP NOT NULL,
                    updated_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (teacher_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- Create class_users table
                CREATE TABLE class_users (
                    id SERIAL PRIMARY KEY,
                    class_id INTEGER NOT NULL,
                    user_id INTEGER NOT NULL,
                    role TEXT NOT NULL,
                    updated_at TIMESTAMP NOT NULL,
                    joined_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- Create homeworks table
                CREATE TABLE homeworks (
                    id SERIAL PRIMARY KEY,
                    class_id INTEGER NOT NULL,
                    created_by INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    content TEXT,
                    attachments TEXT,
                    max_score DOUBLE PRECISION NOT NULL,
                    deadline TIMESTAMP,
                    allow_late_submission BOOLEAN NOT NULL DEFAULT FALSE,
                    created_at TIMESTAMP NOT NULL,
                    updated_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE,
                    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
                );

                -- Create submissions table
                CREATE TABLE submissions (
                    id SERIAL PRIMARY KEY,
                    homework_id INTEGER NOT NULL,
                    creator_id INTEGER NOT NULL,
                    content TEXT NOT NULL,
                    attachments TEXT,
                    submitted_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (homework_id) REFERENCES homeworks(id) ON DELETE CASCADE,
                    FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE
                );

                -- Create grades table
                CREATE TABLE grades (
                    id SERIAL PRIMARY KEY,
                    submission_id INTEGER NOT NULL,
                    grader_id INTEGER NOT NULL,
                    score DOUBLE PRECISION NOT NULL,
                    comment TEXT,
                    graded_at TIMESTAMP NOT NULL,
                    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE,
                    FOREIGN KEY (grader_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- Create files table
                CREATE TABLE files (
                    submission_token TEXT PRIMARY KEY,
                    file_name TEXT NOT NULL,
                    file_size INTEGER NOT NULL,
                    file_type TEXT NOT NULL,
                    uploaded_at TIMESTAMP NOT NULL,
                    citation_count INTEGER DEFAULT 0,
                    user_id INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
                );

                -- Insert initial admin user (username: admin, password: admin123)
                -- Change to random password in production
                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('admin', 'admin@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'admin', 'active', 'Administrator', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                -- Create indexes for users table
                CREATE INDEX idx_users_username ON users(username);
                CREATE INDEX idx_users_email ON users(email);
                CREATE INDEX idx_users_role ON users(role);
                CREATE INDEX idx_users_status ON users(status);
                CREATE INDEX idx_users_last_login ON users(last_login);

                -- Create indexes for classes table
                CREATE INDEX idx_classes_class_name ON classes(class_name);
                CREATE INDEX idx_classes_teacher_id ON classes(teacher_id);
                CREATE INDEX idx_classes_invite_code ON classes(invite_code);

                -- Create indexes for class_users table
                CREATE INDEX idx_class_users_class_id ON class_users(class_id);
                CREATE INDEX idx_class_users_user_id ON class_users(user_id);
                CREATE INDEX idx_class_users_role ON class_users(role);

                -- Create indexes for files table
                CREATE INDEX idx_files_citation_count ON files(citation_count);
                CREATE INDEX idx_files_user_id ON files(user_id);
            "#.to_string(),
        },
        Migration {
            version: 2,
            name: "add_test_data".to_string(),
            up_sql: r#"
                -- Insert test data
                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_1', 'test@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'teacher', 'active', 'Test User', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_2', 'test2@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'teacher', 'active', 'Test User 2', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_3', 'test3@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 3', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_4', 'test4@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 4', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_5', 'test5@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 5', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO users (username, email, password_hash, role, status, profile_name, avatar_url, last_login, created_at, updated_at)
                VALUES ('test_user_6', 'test6@example.com', '$argon2id$v=19$m=65536,t=3,p=4$3pcWjxCi/qihfYIXNadQ0g$uITChD8gDEHSt6eREb/enzd7jmjfOF8KCg+zDBQvMUs', 'user', 'active', 'Test User 6', NULL, NULL, NOW(), NOW())
                ON CONFLICT (username) DO NOTHING;

                INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at)
                VALUES (2, 'Test Class 1', 'This is a test class', 'TEST123', NOW(), NOW());

                INSERT INTO classes (teacher_id, class_name, description, invite_code, created_at, updated_at)
                VALUES (3, 'Test Class 2', 'This is another test class', 'TEST456', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 2, 'teacher', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 4, 'class_representative', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (1, 5, 'student', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 3, 'teacher', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 6, 'class_representative', NOW(), NOW());

                INSERT INTO class_users (class_id, user_id, role, updated_at, joined_at)
                VALUES (2, 7, 'student', NOW(), NOW());

                -- These test data should be removed in production
            "#.to_string(),
        },
    ]
}