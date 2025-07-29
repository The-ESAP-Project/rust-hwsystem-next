use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};
use std::str::FromStr;
use crate::sqlx_enum_type;

// 作业状态
// src/models/homeworks/entities.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HomeworkStatus {
    Pending,    // 待提交
    Expired,    // 已过期
    Submitted,  // 已提交
    Marked,     // 已批改
}

impl FromStr for HomeworkStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(HomeworkStatus::Pending),
            "expired" => Ok(HomeworkStatus::Expired),
            "submitted" => Ok(HomeworkStatus::Submitted),
            "marked" => Ok(HomeworkStatus::Marked),
            "" => Err("状态不能为空".to_string()),
            _ => Err(format!("无效的作业状态: {s}")),
        }
    }
}

impl std::fmt::Display for HomeworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeworkStatus::Pending => write!(f, "pending"),
            HomeworkStatus::Expired => write!(f, "expired"),
            HomeworkStatus::Submitted => write!(f, "submitted"),
            HomeworkStatus::Marked => write!(f, "marked"),
        }
    }
}

sqlx_enum_type!(sqlx::Postgres, sqlx::postgres::PgValueRef<'r>, HomeworkStatus);
sqlx_enum_type!(sqlx::Sqlite, sqlx::sqlite::SqliteValueRef<'r>, HomeworkStatus);

// 附件字段封装
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachments(pub Vec<String>);

impl Attachments {
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }

    pub fn as_inner(&self) -> &Vec<String> {
        &self.0
    }
}

// 创建 JSON 包装类型
#[derive(Debug)]
pub struct JsonAttachments(Option<JsonValue>);

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for JsonAttachments {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json = <Option<JsonValue> as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(JsonAttachments(json))
    }
}

impl sqlx::Type<sqlx::Postgres> for JsonAttachments {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("JSONB")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Attachments {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let json = <Option<JsonValue> as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        if let Some(json) = json {
            let attachments: Vec<String> = serde_json::from_value(json)?;
            Ok(Attachments(attachments))
        } else {
            Ok(Attachments(vec![]))
        }
    }
}

impl sqlx::Type<sqlx::Sqlite> for Attachments {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <str as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

// Homework 实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Homework {
    pub id: i64,
    pub class_id: i64,
    pub title: String,
    pub description: String,
    pub content: Option<String>,

    #[sqlx(rename = "attachments")]
    pub attachments: Option<Attachments>,

    pub max_score: f32,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub allow_late_submission: bool,
    pub created_by: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: HomeworkStatus,
}
