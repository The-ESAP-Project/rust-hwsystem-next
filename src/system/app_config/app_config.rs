use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 应用配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub cors: CorsConfig,
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
    pub secret: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
    pub refresh_token_remember_me_expiry: i64,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub backend: String,
    pub url: String,
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

impl AppConfig {
    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // 首先加载默认配置文件
            .add_source(File::with_name("config").required(false))
            // 然后根据环境加载特定配置文件
            .add_source(
                File::with_name(&format!(
                    "config.{}",
                    std::env::var("APP_ENV").unwrap_or_else(|_| "development".into())
                ))
                .required(false),
            )
            // 最后加载环境变量覆盖
            .add_source(
                Environment::with_prefix("HWSYSTEM")
                    .separator("_")
                    .try_parsing(true),
            );

        // 支持从环境变量加载
        builder = builder
            .set_override_option("app.environment", std::env::var("APP_ENV").ok())?
            .set_override_option("app.log_level", std::env::var("RUST_LOG").ok())?
            .set_override_option("server.host", std::env::var("SERVER_HOST").ok())?
            .set_override_option("server.port", std::env::var("SERVER_PORT").ok())?
            .set_override_option("server.unix_socket_path", std::env::var("UNIX_SOCKET").ok())?
            .set_override_option("server.workers", std::env::var("CPU_COUNT").ok())?
            .set_override_option("jwt.secret", std::env::var("JWT_SECRET").ok())?
            .set_override_option("database.backend", std::env::var("STORAGE_BACKEND").ok())?
            .set_override_option("database.url", std::env::var("DATABASE_URL").ok())?
            .set_override_option("cache.redis.url", std::env::var("REDIS_URL").ok())?
            .set_override_option(
                "cache.redis.key_prefix",
                std::env::var("REDIS_KEY_PREFIX").ok(),
            )?
            .set_override_option("cache.redis.default_ttl", std::env::var("REDIS_TTL").ok())?;

        let config = builder.build()?;
        let mut app_config: AppConfig = config.try_deserialize()?;

        // 处理工作线程数
        if app_config.server.workers == 0 {
            app_config.server.workers = num_cpus::get().min(app_config.server.max_workers);
        }

        Ok(app_config)
    }

    /// 获取全局配置实例
    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                eprintln!("Failed to load configuration: {e}");
                std::process::exit(1);
            })
        })
    }

    /// 初始化配置 (在应用启动时调用)
    pub fn init() -> Result<(), ConfigError> {
        let config = Self::load()?;
        APP_CONFIG
            .set(config)
            .map_err(|_| ConfigError::Message("Configuration already initialized".to_string()))?;
        Ok(())
    }

    /// 检查是否为生产环境
    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }

    /// 检查是否为开发环境
    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }

    /// 获取服务器绑定地址
    pub fn server_bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// 获取 Unix 套接字路径 (如果配置了)
    #[cfg(unix)]
    pub fn unix_socket_path(&self) -> Option<&str> {
        if self.server.unix_socket_path.is_empty() {
            None
        } else {
            Some(&self.server.unix_socket_path)
        }
    }
}
