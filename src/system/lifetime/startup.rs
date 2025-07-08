use crate::cache::{ObjectCache, register::get_object_cache_plugin};
use crate::storages::{Storage, StorageFactory};
use std::sync::Arc;
use tracing::warn;

pub struct StartupContext {
    pub storage: Arc<dyn Storage>,
    pub cache: Arc<dyn ObjectCache>,
}

/// 创建缓存实例
async fn create_cache() -> Result<Arc<dyn ObjectCache>, Box<dyn std::error::Error>> {
    // 优先尝试 Redis 缓存
    if let Some(constructor) = get_object_cache_plugin("redis") {
        match constructor().await {
            Ok(cache) => {
                warn!("Using Redis cache backend");
                return Ok(Arc::from(cache));
            }
            Err(e) => {
                warn!("Failed to create Redis cache: {}, falling back to Moka", e);
            }
        }
    }

    // 回退到内存缓存
    if let Some(constructor) = get_object_cache_plugin("moka") {
        match constructor().await {
            Ok(cache) => {
                warn!("Using Moka (in-memory) cache backend");
                return Ok(Arc::from(cache));
            }
            Err(e) => {
                warn!("Failed to create Moka cache: {}", e);
            }
        }
    }

    Err("No cache backend available".into())
}

/// 准备服务器启动的上下文
/// 包括存储、缓存和路由配置等
pub async fn prepare_server_startup() -> StartupContext {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    if cfg!(debug_assertions) {
        crate::storages::register::debug_storage_registry();
        crate::cache::register::debug_object_cache_registry();
        warn!("Debug mode: Storage and cache registries are enabled");
    }

    let storage = StorageFactory::create()
        .await
        .expect("Failed to create storage backend");
    warn!("Storage backend initialized and migrations completed");

    // 创建缓存实例
    let cache = create_cache().await.expect("Failed to create cache");
    warn!("Cache backend initialized");

    StartupContext { storage, cache }
}
