use sqlx::FromRow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromRow)]
pub struct Homework {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub deadline: Option<i64>,
    pub max_score: i32,
    pub allow_late_submission: i32,
    pub attachments: Option<String>,
    pub submission_count: i32,
    pub status: String,
    pub created_by: i64,
    pub created_at: i64,
    pub updated_at: i64,
}