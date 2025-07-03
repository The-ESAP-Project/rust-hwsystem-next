use serde::{Deserialize, Serialize};

// 用户角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    #[serde(rename = "teacher")]
    Teacher, // 教师
    #[serde(rename = "student")]
    Student, // 学生
    #[serde(rename = "admin")]
    Admin, // 管理员
    #[serde(rename = "class_representative")]
    ClassRepresentative, // 课代表
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active, // 活跃
    #[serde(rename = "inactive")]
    Inactive, // 非活跃
    #[serde(rename = "suspended")]
    Suspended, // 暂停
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
    pub role: UserRole,
    pub status: UserStatus,
    pub profile: Option<UserProfile>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
