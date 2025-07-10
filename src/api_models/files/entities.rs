use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct File {
    pub id: i64,
    pub unique_name: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i64,
}
