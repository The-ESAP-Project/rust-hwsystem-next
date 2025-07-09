use serde::Deserialize;

// 用户登录请求（来自HTTP请求）
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// 用户名或邮箱
    pub username: String,
    /// 密码
    pub password: String,
    /// 是否记住我
    #[serde(default)]
    pub remember_me: bool,
}
