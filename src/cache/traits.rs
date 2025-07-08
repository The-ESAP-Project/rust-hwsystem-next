use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};

/// 缓存查询结果
#[derive(Debug, Clone)]
pub enum CacheResult<T> {
    /// 确定不存在
    NotFound,
    /// 存在但没有缓存值
    ExistsButNoValue,
    /// 成功获取到缓存值
    Found(T),
}

/// 对象缓存 trait，直接操作 JSON 字符串，类型安全由上层保证
#[async_trait]
pub trait ObjectCache: Send + Sync {
    /// 获取原始 JSON 字符串
    async fn get_raw(&self, key: &str) -> CacheResult<String>;

    /// 插入原始 JSON 字符串
    async fn insert_raw(&self, key: String, value: String, ttl: u32);

    /// 移除指定键
    async fn remove(&self, key: &str);

    /// 清空所有缓存
    async fn invalidate_all(&self);
}

/// 类型安全的缓存扩展 trait
pub trait TypedObjectCache {
    /// 获取指定类型的缓存值
    fn get<T>(&self, key: &str) -> impl std::future::Future<Output = CacheResult<T>> + Send
    where
        T: for<'de> Deserialize<'de> + Send;

    /// 插入指定类型的缓存值（带 TTL）
    fn insert<T>(
        &self,
        key: String,
        value: T,
        ttl: u32,
    ) -> impl std::future::Future<Output = ()> + Send
    where
        T: Serialize + Send;

    /// 插入指定类型的缓存值（使用默认 TTL）
    fn insert_with_default_ttl<T>(
        &self,
        key: String,
        value: T,
    ) -> impl std::future::Future<Output = ()> + Send
    where
        T: Serialize + Send;

    /// 批量加载缓存
    fn load_l2_cache<T>(
        &self,
        keys: DashMap<String, T>,
    ) -> impl std::future::Future<Output = ()> + Send
    where
        T: Serialize + Send + Sync + Clone;
}

// 为所有实现了 ObjectCache 的类型自动实现 TypedObjectCache
impl<C: ObjectCache + ?Sized> TypedObjectCache for C {
    async fn get<T>(&self, key: &str) -> CacheResult<T>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        match self.get_raw(key).await {
            CacheResult::Found(json) => match serde_json::from_str(&json) {
                Ok(value) => CacheResult::Found(value),
                Err(e) => {
                    tracing::warn!(
                        "Failed to deserialize cached value for key '{}': {}",
                        key,
                        e
                    );
                    CacheResult::ExistsButNoValue
                }
            },
            CacheResult::NotFound => CacheResult::NotFound,
            CacheResult::ExistsButNoValue => CacheResult::ExistsButNoValue,
        }
    }

    async fn insert<T>(&self, key: String, value: T, ttl: u32) -> ()
    where
        T: Serialize + Send,
    {
        match serde_json::to_string(&value) {
            Ok(json) => {
                self.insert_raw(key, json, ttl).await;
            }
            Err(e) => {
                tracing::error!("Failed to serialize value for key '{}': {}", key, e);
            }
        }
    }

    async fn insert_with_default_ttl<T>(&self, key: String, value: T) -> ()
    where
        T: Serialize + Send,
    {
        self.insert(key, value, 0).await // TTL 为 0 表示使用默认 TTL
    }

    async fn load_l2_cache<T>(&self, keys: DashMap<String, T>) -> ()
    where
        T: Serialize + Send + Sync,
    {
        for entry in keys.iter() {
            // 克隆值以避免引用问题
            let key = entry.key().clone();
            let value = entry.value();
            self.insert(key, value, 0).await; // 默认 TTL 为 0，表示不设置过期时间
        }
    }
}
