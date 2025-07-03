#[macro_use]
mod macros;

use std::env;
use std::sync::Arc;
use tracing::error;

use crate::errors::{HWSystemError, Result};

pub mod backends;
pub mod models;
pub mod register;

pub use models::{CreateUserRequest, UpdateUserRequest, User, UserRole, UserStatus};
use register::get_storage_plugin;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    // 用户管理方法
    async fn create_user(&self, user: CreateUserRequest) -> Result<User>;
    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn search_users(
        &self,
        query: &str,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>>;
    async fn list_users(
        &self,
        role: Option<UserRole>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>>;
    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>>;
    async fn delete_user(&self, id: i64) -> Result<bool>;

    // 数据库迁移方法
    async fn run_migrations(&self) -> Result<()>;
    async fn get_schema_version(&self) -> Result<i32>;
}

pub struct StorageFactory;

impl StorageFactory {
    pub async fn create() -> Result<Arc<dyn Storage>> {
        let backend = env::var("STORAGE_BACKEND").unwrap_or_else(|_| "sqlite".into());

        if let Some(ctor) = get_storage_plugin(&backend) {
            let storage = ctor().await?;
            Ok(Arc::from(storage))
        } else {
            error!("Failed to create storage backend: {}", backend);
            let available_backends = register::get_storage_plugin_names();
            error!("Available storage backends: {:?}", available_backends);
            Err(HWSystemError::storage_plugin_not_found(format!(
                "Unknown storage backend: {backend}"
            )))
        }
    }
}
