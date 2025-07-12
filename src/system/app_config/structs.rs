use serde::{Deserialize, Serialize};

/// 应用配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub cors: CorsConfig,
    pub upload: UploadConfig,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub environment: String,
    pub log_level: String,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub unix_socket_path: String,
    pub workers: usize,
    pub max_workers: usize,
    pub timeouts: TimeoutConfig,
    pub limits: LimitConfig,
}

/// 超时配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    pub client_request: u64,
    pub client_disconnect: u64,
    pub keep_alive: u64,
}

/// 限制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitConfig {
    pub max_payload_size: usize,
}

/// JWT 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    #[serde(skip_serializing, default)] // 不序列化到JSON响应中
    pub secret: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
    pub refresh_token_remember_me_expiry: i64,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    #[serde(rename = "type")]
    pub backend: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub pool_size: u32,
    pub timeout: u64,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(rename = "type")]
    pub cache_type: String,
    pub redis: RedisConfig,
    pub memory: MemoryConfig,
}

/// Redis 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub key_prefix: String,
    pub default_ttl: u64,
    pub pool_size: u32,
}

/// 内存缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub max_capacity: u64,
    pub default_ttl: u64,
}

/// CORS 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    pub dir: String,                // 上传目录
    pub max_size: usize,            // 单文件最大字节数
    pub allowed_types: Vec<String>, // 允许的MIME类型或扩展名
}
