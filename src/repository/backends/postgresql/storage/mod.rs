pub mod class_users;
pub mod file;
pub mod homeworks;
pub mod storage_impl;
pub mod user;
mod classes;

use super::migrations::PostgresqlMigrationManager;
use crate::errors::{HWSystemError, Result};
use crate::system::app_config::AppConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{error, warn};

#[derive(Clone)]
pub struct PostgresqlStorage {
    pool: PgPool,
}

impl PostgresqlStorage {
    pub async fn new_async() -> Result<Self> {
        let config = AppConfig::get();

        // 创建连接池
        let pool = PgPoolOptions::new()
            .max_connections(config.database.pool_size)
            .acquire_timeout(std::time::Duration::from_secs(config.database.timeout))
            .connect(&config.database.url)
            .await
            .map_err(|e| {
                HWSystemError::database_config(format!("Could not connect to database: {e}"))
            })?;

        // 测试连接
        if let Err(e) = sqlx::query("SELECT 1").execute(&pool).await {
            error!("Database connection test failed: {}", e);
            return Err(HWSystemError::database_connection(format!(
                "Database connection test failed: {e}"
            )));
        }

        let storage = PostgresqlStorage { pool };

        // 初始化迁移系统并运行迁移
        let migration_manager = PostgresqlMigrationManager::new(storage.pool.clone());
        migration_manager.init().await?;
        migration_manager.migrate_up().await?;

        warn!("PostgreSQL database connection successful");

        Ok(storage)
    }
}

// 注册 PostgreSQL 存储插件
declare_storage_plugin!("postgresql", PostgresqlStorage);
