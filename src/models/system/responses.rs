use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemSettingsResponse {
    pub system_name: String,             // 系统名称
    pub max_file_size: u64,              // 单文件最大字节数
    pub allowed_file_types: Vec<String>, // 允许的文件类型
    pub environment: String,             // 运行环境
    pub log_level: String,               // 日志级别
}
