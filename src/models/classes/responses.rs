use super::entities::Class;
use crate::models::common::PaginationInfo;
use serde::Serialize;

// 班级响应模型
#[derive(Debug, Serialize)]
pub struct ClassResponse {
    pub id: i64,
    pub class_name: String,
    pub description: Option<String>,
    pub teacher_id: i64,
    pub invite_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Class> for ClassResponse {
    fn from(class: Class) -> Self {
        Self {
            id: class.id,
            class_name: class.class_name,
            description: class.description,
            teacher_id: class.teacher_id,
            invite_code: class.invite_code,
            created_at: class.created_at,
            updated_at: class.updated_at,
        }
    }
}

// 班级列表响应
#[derive(Debug, Serialize)]
pub struct ClassListResponse {
    pub items: Vec<ClassResponse>,
    pub pagination: PaginationInfo,
}
