use async_trait::async_trait;
use moka::future::Cache;

use crate::cache::{CacheResult, ObjectCache};
use crate::declare_object_cache_plugin;

declare_object_cache_plugin!("moka", MokaCacheWrapper);

pub struct MokaCacheWrapper {
    inner: Cache<String, String>,
}

impl Default for MokaCacheWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl MokaCacheWrapper {
    pub fn new() -> Self {
        let inner = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(std::time::Duration::from_secs(900))
            .build();
        Self { inner }
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
