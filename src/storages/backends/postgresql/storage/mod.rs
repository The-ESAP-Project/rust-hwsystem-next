pub mod classes;
pub mod file;
pub mod homeworks;
pub mod storage_impl;
pub mod user;

use super::migrations::PostgresqlMigrationManager;
use crate::errors::{HWSystemError, Result};
use crate::system::app_config::AppConfig;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct PostgresqlStorage {
    pool: PgPool,
}

impl PostgresqlStorage {
    pub async fn new_async() -> Result<Self> {
        let config = AppConfig::get();

        info!("初始化 PostgreSQL 数据库连接...");
        info!(
            "数据库信息: {}:{}/{}",
            config.database.host, config.database.port, config.database.name
        );
        info!("连接池配置: {} 连接", config.database.pool_size);

        // 创建连接池
        let pool = PgPoolOptions::new()
            .max_connections(config.database.pool_size)
            .acquire_timeout(std::time::Duration::from_secs(config.database.timeout))
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                config.database.user,
                config.database.password,
                config.database.host,
                config.database.port,
                config.database.name
            ))
            .await
            .map_err(|e| HWSystemError::database_connection(format!("无法连接到数据库: {e}")))?;

        // 测试连接
        if let Err(e) = sqlx::query("SELECT 1").execute(&pool).await {
            error!("数据库连接测试失败: {}", e);
            return Err(HWSystemError::database_connection(format!(
                "数据库连接测试失败: {e}"
            )));
        }

        let storage = PostgresqlStorage { pool };

        // 初始化迁移系统并运行迁移
        let migration_manager = PostgresqlMigrationManager::new(storage.pool.clone());
        migration_manager.init().await?;
        migration_manager.migrate_up().await?;

        info!("PostgreSQL 数据库连接成功");
        warn!(
            "PostgresqlStorage initialized, database: {}:{}/{}",
            config.database.host, config.database.port, config.database.name
        );

        Ok(storage)
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

// 注册 PostgreSQL 存储插件
declare_storage_plugin!("postgresql", PostgresqlStorage);
