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
pub struct ClassStudent {
    pub id: i64,
    pub class_id: i64,
    pub student_id: i64,
    pub role: ClassUserRole,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

impl ClassStudent {
    pub fn from_row_prefix(
        prefix: &str,
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(&*format!("{prefix}id"))?,
            class_id: row.try_get(&*format!("{prefix}class_id"))?,
            student_id: row.try_get(&*format!("{prefix}student_id"))?,
            role: row.try_get(&*format!("{prefix}role"))?,
            joined_at: row.try_get(&*format!("{prefix}joined_at"))?,
        })
    }
}
