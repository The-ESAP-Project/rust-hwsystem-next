use serde::{Deserialize, Serialize};

// 用户角色
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Teacher,             // 教师
    Student,             // 学生
    Admin,               // 管理员
    ClassRepresentative, // 课代表
}

impl<'de> Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "teacher" => Ok(UserRole::Teacher),
            "student" => Ok(UserRole::Student),
            "admin" => Ok(UserRole::Admin),
            "class_representative" => Ok(UserRole::ClassRepresentative),
            _ => Err(serde::de::Error::custom(format!(
                "无效的用户角色: '{s}'. 支持的角色: teacher, student, admin, class_representative"
            ))),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Teacher => write!(f, "teacher"),
            UserRole::Student => write!(f, "student"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::ClassRepresentative => write!(f, "class_representative"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "teacher" => Ok(UserRole::Teacher),
            "student" => Ok(UserRole::Student),
            "admin" => Ok(UserRole::Admin),
            "class_representative" => Ok(UserRole::ClassRepresentative),
            _ => Err(format!("Invalid user role: {s}")),
        }
    }
}

// 用户状态
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,    // 活跃
    Inactive,  // 非活跃
    Suspended, // 暂停
}

impl<'de> Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            _ => Err(serde::de::Error::custom(format!(
                "无效的用户状态: '{s}'. 支持的状态: active, inactive, suspended"
            ))),
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Suspended => write!(f, "suspended"),
        }
    }
}

impl std::str::FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            _ => Err(format!("Invalid user status: {s}")),
        }
    }
}

// 用户资料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub student_id: Option<String>,
    pub class: Option<String>,
    pub avatar_url: Option<String>,
}

// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // 不序列化到JSON响应中
    pub password_hash: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub profile: Option<UserProfile>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    // 生成访问令牌（使用真正的 JWT）
    pub async fn generate_access_token(&self) -> String {
        // 使用 JwtUtils 生成 access token
        match crate::utils::jwt::JwtUtils::generate_access_token(self.id, &self.role.to_string()) {
            Ok(token) => token,
            Err(e) => {
                // 如果 JWT 生成失败，返回一个简单的 token（不推荐在生产环境中使用）
                tracing::error!("JWT token 生成失败: {}", e);
                format!(
                    "fallback_token_{}_{}",
                    self.id,
                    chrono::Utc::now().timestamp()
                )
            }
        }
    }

    // 生成刷新令牌
    pub async fn generate_refresh_token(&self) -> String {
        match crate::utils::jwt::JwtUtils::generate_refresh_token(self.id, &self.role.to_string()) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("JWT refresh token 生成失败: {}", e);
                format!(
                    "fallback_refresh_token_{}_{}",
                    self.id,
                    chrono::Utc::now().timestamp()
                )
            }
        }
    }

    // 生成 token 对（access + refresh）
    pub async fn generate_token_pair(&self) -> Result<crate::utils::jwt::TokenPair, String> {
        crate::utils::jwt::JwtUtils::generate_token_pair(self.id, &self.role.to_string())
            .map_err(|e| format!("生成 token 对失败: {e}"))
    }
}
