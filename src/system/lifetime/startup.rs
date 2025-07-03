use crate::storages::{Storage, StorageFactory};
use std::sync::Arc;
use tracing::warn;

pub struct StartupContext {
    pub storage: Arc<dyn Storage>,
}

/// 准备服务器启动的上下文
/// 包括存储、缓存和路由配置等
pub async fn prepare_server_startup() -> StartupContext {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    if cfg!(debug_assertions) {
        crate::storages::register::debug_storage_registry();
    }

    let storage = StorageFactory::create()
        .await
        .expect("Failed to create storage backend");
    warn!("Storage backend initialized and migrations completed");

    StartupContext { storage }
}
