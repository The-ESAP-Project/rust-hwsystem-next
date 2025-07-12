use std::fmt::{self};

#[derive(Debug, Clone)]
pub enum HWSystemError {
    CacheConnection(String),
    DatabaseConfig(String),
    DatabaseConnection(String),
    DatabaseOperation(String),
    FileOperation(String),
    Validation(String),
    Serialization(String),
    StoragePluginNotFound(String),
    DateParse(String),
}

impl fmt::Display for HWSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HWSystemError::CacheConnection(msg) => write!(f, "缓存连接错误: {msg}"),
            HWSystemError::DatabaseConfig(msg) => write!(f, "数据库配置错误: {msg}"),
            HWSystemError::DatabaseConnection(msg) => write!(f, "数据库连接错误: {msg}"),
            HWSystemError::DatabaseOperation(msg) => write!(f, "数据库操作错误: {msg}"),
            HWSystemError::FileOperation(msg) => write!(f, "文件操作错误: {msg}"),
            HWSystemError::Validation(msg) => write!(f, "验证错误: {msg}"),
            HWSystemError::Serialization(msg) => write!(f, "序列化错误: {msg}"),
            HWSystemError::StoragePluginNotFound(msg) => write!(f, "存储插件未找到: {msg}"),
            HWSystemError::DateParse(msg) => write!(f, "日期解析错误: {msg}"),
        }
    }
}

impl std::error::Error for HWSystemError {}

// 便捷的构造函数
impl HWSystemError {
    pub fn cache_connection<T: Into<String>>(msg: T) -> Self {
        HWSystemError::CacheConnection(msg.into())
    }

    pub fn database_config<T: Into<String>>(msg: T) -> Self {
        HWSystemError::DatabaseConfig(msg.into())
    }

    pub fn database_connection<T: Into<String>>(msg: T) -> Self {
        HWSystemError::DatabaseConnection(msg.into())
    }

    pub fn database_operation<T: Into<String>>(msg: T) -> Self {
        HWSystemError::DatabaseOperation(msg.into())
    }

    pub fn file_operation<T: Into<String>>(msg: T) -> Self {
        HWSystemError::FileOperation(msg.into())
    }

    pub fn validation<T: Into<String>>(msg: T) -> Self {
        HWSystemError::Validation(msg.into())
    }

    pub fn serialization<T: Into<String>>(msg: T) -> Self {
        HWSystemError::Serialization(msg.into())
    }

    pub fn storage_plugin_not_found<T: Into<String>>(msg: T) -> Self {
        HWSystemError::StoragePluginNotFound(msg.into())
    }

    pub fn date_parse<T: Into<String>>(msg: T) -> Self {
        HWSystemError::DateParse(msg.into())
    }
}

// 为常见的错误类型实现 From trait
impl From<sqlx::Error> for HWSystemError {
    fn from(err: sqlx::Error) -> Self {
        HWSystemError::DatabaseOperation(err.to_string())
    }
}

impl From<std::io::Error> for HWSystemError {
    fn from(err: std::io::Error) -> Self {
        HWSystemError::FileOperation(err.to_string())
    }
}

impl From<serde_json::Error> for HWSystemError {
    fn from(err: serde_json::Error) -> Self {
        HWSystemError::Serialization(err.to_string())
    }
}

impl From<chrono::ParseError> for HWSystemError {
    fn from(err: chrono::ParseError) -> Self {
        HWSystemError::DateParse(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, HWSystemError>;
