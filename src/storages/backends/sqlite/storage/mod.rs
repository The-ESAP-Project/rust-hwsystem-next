pub mod classes;
pub mod file;
pub mod homeworks;
pub mod storage_impl;
pub mod user;

use super::migrations::SqliteMigrationManager;
use crate::errors::{HWSystemError, Result};
use crate::system::app_config::AppConfig;
use sqlx::{SqlitePool, sqlite, sqlite::SqliteConnectOptions};
use tracing::warn;

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new_async() -> Result<Self> {
        let config = AppConfig::get();

        // 创建连接池
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::new()
                .filename(&config.database.url)
                .create_if_missing(true)
                .journal_mode(sqlite::SqliteJournalMode::Wal)
                .synchronous(sqlite::SqliteSynchronous::Normal)
                .busy_timeout(std::time::Duration::from_secs(config.database.timeout))
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

        warn!(
            "SqliteStorage initialized, database path: {}",
            &config.database.url
        );

        Ok(storage)
    }
}

// 注册 SQLite 存储插件
declare_storage_plugin!("sqlite", SqliteStorage);
