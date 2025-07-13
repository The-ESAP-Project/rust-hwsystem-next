use async_trait::async_trait;
use moka::future::Cache;
use tracing::debug;

use crate::cache::{CacheResult, ObjectCache};
use crate::declare_object_cache_plugin;
use crate::system::app_config::AppConfig;

declare_object_cache_plugin!("moka", MokaCacheWrapper);

pub struct MokaCacheWrapper {
    inner: Cache<String, String>,
}

impl Default for MokaCacheWrapper {
    fn default() -> Self {
        Self::new().expect("MokaCacheWrapper 初始化失败，请检查配置")
    }
}

impl MokaCacheWrapper {
    pub fn new() -> Result<Self, String> {
        let config = AppConfig::get();
        let inner = Cache::builder()
            .max_capacity(config.cache.memory.max_capacity)
            .time_to_live(std::time::Duration::from_secs(
                config.cache.memory.default_ttl,
            ))
            .build();

        debug!(
            "MokaCacheWrapper initialized with max capacity: {}",
            config.cache.memory.max_capacity
        );
        Ok(Self { inner })
    }
}

#[async_trait]
impl ObjectCache for MokaCacheWrapper {
    async fn get_raw(&self, key: &str) -> CacheResult<String> {
        if let Some(value) = self.inner.get(key).await {
            CacheResult::Found(value)
        } else {
            CacheResult::NotFound
        }
    }

    async fn insert_raw(&self, key: String, value: String, ttl: u32) {
        // Moka 缓存使用配置的 TTL，这里的 ttl 参数会被忽略
        // 因为 Moka 在创建时就设置了全局的 TTL 策略
        self.inner.insert(key, value).await;

        // 如果需要支持动态 TTL，可以考虑使用 insert_with_weight_and_ttl
        // 但这需要 Moka 的更高级功能
        if ttl != 0 {
            tracing::debug!("Moka cache ignores per-item TTL, using global TTL configuration");
        }
    }

    async fn remove(&self, key: &str) {
        self.inner.invalidate(key).await;
    }

    async fn invalidate_all(&self) {
        self.inner.invalidate_all();
    }
}
