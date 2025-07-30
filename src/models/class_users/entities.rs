use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::sqlx_enum_type;
use sqlx::Row;

// 用户角色
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClassUserRole {
    Student,             // 学生
    ClassRepresentative, // 课代表
    Teacher,             // 教师
}

impl ClassUserRole {
    pub const STUDENT: &'static str = "student";
    pub const TEACHER: &'static str = "teacher";
    pub const CLASSREPRESENTATIVE: &'static str = "class_representative";

    pub fn class_teacher_roles() -> &'static [&'static ClassUserRole] {
        &[&Self::Teacher]
    }
    pub fn class_representative_roles() -> &'static [&'static ClassUserRole] {
        &[&Self::ClassRepresentative, &Self::Teacher]
    }
    pub fn all_roles() -> &'static [&'static ClassUserRole] {
        &[&Self::Student, &Self::ClassRepresentative, &Self::Teacher]
    }
}

impl<'de> Deserialize<'de> for ClassUserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "student" => Ok(ClassUserRole::Student),
            "class_representative" => Ok(ClassUserRole::ClassRepresentative),
            "teacher" => Ok(ClassUserRole::Teacher),
            _ => Err(serde::de::Error::custom(format!(
                "无效的班级用户角色: '{s}'. 支持的角色: student, class_representative, teacher"
            ))),
        }
    }
}

impl std::fmt::Display for ClassUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassUserRole::Student => write!(f, "student"),
            ClassUserRole::ClassRepresentative => write!(f, "class_representative"),
            ClassUserRole::Teacher => write!(f, "teacher"),
        }
    }
}

impl std::str::FromStr for ClassUserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "student" => Ok(ClassUserRole::Student),
            "class_representative" => Ok(ClassUserRole::ClassRepresentative),
            "teacher" => Ok(ClassUserRole::Teacher),
            _ => Err(format!("Invalid class user role: {s}")),
        }
    }
}

// 分别为 PostgreSQL 和 SQLite 实现
sqlx_enum_type!(
    sqlx::Postgres,
    sqlx::postgres::PgValueRef<'r>,
    ClassUserRole
);
sqlx_enum_type!(
    sqlx::Sqlite,
    sqlx::sqlite::SqliteValueRef<'r>,
    ClassUserRole
);

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClassUser {
    pub id: i64,
    pub class_id: i64,
    pub user_id: i64,
    pub profile_name: Option<String>,
    pub role: ClassUserRole,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

impl ClassUser {
    pub fn from_row_prefix<'r, R>(prefix: &str, row: &'r R) -> Result<Self, sqlx::Error>
    where
        R: sqlx::Row,
        ClassUserRole: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        i64: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        String: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        chrono::DateTime<chrono::Utc>: sqlx::Type<R::Database> + sqlx::Decode<'r, R::Database>,
        for<'c> &'c str: sqlx::ColumnIndex<R>,
    {
        Ok(Self {
            id: row.try_get(format!("{prefix}id").as_str())?,
            class_id: row.try_get(format!("{prefix}class_id").as_str())?,
            user_id: row.try_get(format!("{prefix}user_id").as_str())?,
            profile_name: row.try_get(format!("{prefix}profile_name").as_str())?,
            role: row.try_get(format!("{prefix}role").as_str())?,
            updated_at: row.try_get(format!("{prefix}updated_at").as_str())?,
            joined_at: row.try_get(format!("{prefix}joined_at").as_str())?,
        })
    }
}
