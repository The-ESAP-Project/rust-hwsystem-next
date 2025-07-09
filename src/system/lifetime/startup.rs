use crate::cache::{ObjectCache, register::get_object_cache_plugin};
use crate::storages::{Storage, StorageFactory};
use crate::system::config::AppConfig;
use std::sync::Arc;
use tracing::warn;

pub struct StartupContext {
    pub storage: Arc<dyn Storage>,
    pub cache: Arc<dyn ObjectCache>,
}

/// 创建缓存实例
async fn create_cache() -> Result<Arc<dyn ObjectCache>, Box<dyn std::error::Error>> {
    let config = AppConfig::get();
    let cache_type = &config.cache.cache_type;
    
    warn!("Attempting to create {} cache backend", cache_type);
    
    // 根据配置选择缓存后端
    if let Some(constructor) = get_object_cache_plugin(cache_type) {
        match constructor().await {
            Ok(cache) => {
                warn!("Successfully created {} cache backend", cache_type);
                return Ok(Arc::from(cache));
            }
            Err(e) => {
                warn!("Failed to create {} cache: {}", cache_type, e);
                
                // 如果配置的缓存失败，尝试回退策略
                if cache_type == "redis" {
                    warn!("Falling back to memory cache");
                    if let Some(fallback_constructor) = get_object_cache_plugin("moka") {
                        match fallback_constructor().await {
                            Ok(cache) => {
                                warn!("Successfully created fallback Moka (in-memory) cache backend");
                                return Ok(Arc::from(cache));
                            }
                            Err(fallback_e) => {
                                warn!("Failed to create fallback Moka cache: {}", fallback_e);
                            }
                        }
                    }
                }
            }
        }
    } else {
        warn!("Cache backend '{}' not found in registry", cache_type);
        
        // 如果找不到配置的缓存类型，尝试默认的内存缓存
        if cache_type != "moka" {
            warn!("Falling back to default memory cache");
            if let Some(fallback_constructor) = get_object_cache_plugin("moka") {
                match fallback_constructor().await {
                    Ok(cache) => {
                        warn!("Successfully created fallback Moka (in-memory) cache backend");
                        return Ok(Arc::from(cache));
                    }
                    Err(fallback_e) => {
                        warn!("Failed to create fallback Moka cache: {}", fallback_e);
                    }
                }
            }
        }
    }

    Err(format!("No cache backend available (tried: {})", cache_type).into())
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
