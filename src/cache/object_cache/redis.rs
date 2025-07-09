use async_trait::async_trait;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use tracing::{debug, error, warn};

use crate::cache::{CacheResult, ObjectCache};
use crate::declare_object_cache_plugin;
use crate::system::config::AppConfig;

declare_object_cache_plugin!("redis", RedisObjectCache);

pub struct RedisObjectCache {
    client: redis::Client,
    key_prefix: String,
    ttl: u64, // TTL in seconds
}

impl RedisObjectCache {
    pub fn new() -> Self {
        let config = AppConfig::get();
        let redis_config = &config.cache.redis;

        debug!(
            "RedisObjectCache created with prefix: '{}', TTL: {}s",
            redis_config.key_prefix, redis_config.default_ttl
        );

        let client = redis::Client::open(redis_config.url.clone())
            .expect("Failed to create Redis client. Check Redis URL in config.");

        // 测试 Redis 连接 - 使用同步连接进行简单测试
        match client.get_connection() {
            Ok(mut conn) => match redis::cmd("PING").query::<String>(&mut conn) {
                Ok(response) => {
                    debug!("Redis connection test successful: {}", response);
                }
                Err(e) => {
                    panic!(
                        "Redis PING command failed: {e}. Check Redis server status and URL: {}",
                        redis_config.url
                    );
                }
            },
            Err(e) => {
                panic!(
                    "Failed to connect to Redis: {e}. Check Redis server status and URL: {}",
                    redis_config.url
                );
            }
        }

        Self {
            client,
            key_prefix: redis_config.key_prefix.clone(),
            ttl: redis_config.default_ttl,
        }
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection, redis::RedisError> {
        let client = &self.client;
        let conn = client.get_multiplexed_tokio_connection().await?;
        Ok(conn)
    }

    fn make_key(&self, key: &str) -> String {
        format!("{}{}", self.key_prefix, key)
    }
}

#[async_trait]
impl ObjectCache for RedisObjectCache {
    async fn get_raw(&self, key: &str) -> CacheResult<String> {
        let redis_key = self.make_key(key);

        let mut conn = match self.get_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {}", e);
                return CacheResult::ExistsButNoValue;
            }
        };

        let result: redis::RedisResult<Option<String>> = conn.get(redis_key).await;

        match result {
            Ok(Some(data)) => {
                debug!("Successfully retrieved key: {}", key);
                CacheResult::Found(data)
            }
            Ok(None) => {
                debug!("Key not found in cache: {}", key);
                CacheResult::NotFound
            }
            Err(e) => {
                error!("Failed to get key '{}': {}", key, e);
                CacheResult::ExistsButNoValue
            }
        }
    }

    async fn insert_raw(&self, key: String, value: String, ttl: u32) {
        let redis_key = self.make_key(&key);

        let mut conn = match self.get_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {}", e);
                return;
            }
        };

        // 使用传入的 TTL，如果为 0 则使用默认 TTL
        let effective_ttl = if ttl == 0 { self.ttl } else { ttl as u64 };

        match conn
            .set_ex::<String, String, ()>(redis_key, value, effective_ttl)
            .await
        {
            Ok(_) => {
                debug!(
                    "Successfully inserted key into cache: {} (TTL: {}s)",
                    key, effective_ttl
                );
            }
            Err(e) => {
                error!("Failed to insert key '{}' into cache: {}", key, e);
            }
        }
    }

    async fn remove(&self, key: &str) {
        let redis_key = self.make_key(key);

        let mut conn = match self.get_connection().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection: {}", e);
                return;
            }
        };

        match conn.del::<String, i32>(redis_key).await {
            Ok(deleted_count) => {
                if deleted_count > 0 {
                    debug!("Successfully removed key from cache: {}", key);
                } else {
                    debug!("Key not found in cache for removal: {}", key);
                }
            }
            Err(e) => {
                error!("Failed to remove key '{}': {}", key, e);
            }
        }
    }

    async fn invalidate_all(&self) {
        warn!("RedisObjectCache does not implement invalidate_all");
        return;
    }
}
