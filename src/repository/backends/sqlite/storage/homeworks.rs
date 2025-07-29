use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::homeworks::entities::HomeworkStatus;
use crate::models::users::entities::UserRole;
use crate::models::{
    PaginationInfo,
    homeworks::{entities::Homework, requests::HomeworkListQuery, responses::HomeworkListResponse},
};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row};
use std::str::FromStr;
pub async fn list_homeworks_with_pagination(
    storage: &SqliteStorage,
    user_id: i64,
    user_role: &UserRole,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    let page = query.pagination.page.max(1);
    let size = query.pagination.size.clamp(1, 100);
    let offset = (page - 1) * size;

    let mut conditions = Vec::new();
    let mut params = Vec::new();

    // 根据角色添加条件
    match user_role {
        UserRole::Admin => {
            // 管理员可以查看所有作业，不需要额外条件
        }
        UserRole::Teacher => {
            // 教师只能查看其所在班级的作业
            conditions.push("h.class_id IN (SELECT class_id FROM class_users WHERE user_id = ? AND role = 'teacher')");
            params.push(user_id.to_string());
        }
        UserRole::User => {
            // 学生只能查看其所在班级的作业
            conditions.push("h.class_id IN (SELECT class_id FROM class_users WHERE user_id = ?)");
            params.push(user_id.to_string());
        }
    }

    // 处理状态筛选
    if let Some(status) = &query.status {
        if !status.is_empty() {
            if let Ok(valid_status) = HomeworkStatus::from_str(status) {
                conditions.push("h.status = ?");
                params.push(valid_status.to_string());
            }
        }
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 构建并执行查询
    let count_sql = format!(
        "SELECT COUNT(DISTINCT h.id) as total
         FROM homeworks h
         {}",
        where_clause
    );

    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total: i64 = count_query
        .fetch_one(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to count homeworks: {e}")))?
        .get(0);

    let query_sql = format!(
        "SELECT DISTINCT h.*
         FROM homeworks h
         {}
         ORDER BY h.created_at DESC LIMIT ? OFFSET ?",
        where_clause
    );

    let mut query = sqlx::query_as::<_, Homework>(&query_sql);
    for param in &params {
        query = query.bind(param);
    }
    query = query.bind(size).bind(offset);

    let items = query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to count homeworks: {e}")))?;

    Ok(HomeworkListResponse {
        items,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages: (total + size - 1) / size,
        },
    })
}
