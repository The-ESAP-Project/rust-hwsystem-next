use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Homework {
    // 唯一 ID
    pub id: i64,
    // 关联的班级 ID
    pub class_id: i64,
    // 作业标题
    pub title: String,
    // 作业描述
    pub description: String,
    // 作业内容
    pub content: Option<String>,
    // 作业附件
    pub attachments: Json<Vec<String>>,
    // 作业最高分数
    pub max_score: f32,
    // 作业截止时间
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    // 是否允许迟交
    pub allow_late_submission: bool,
    // 创建者 ID
    pub created_by: i64,
    // 作业创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    // 作业更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // 作业状态
    pub status: String,
}
