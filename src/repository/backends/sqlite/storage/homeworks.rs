use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::{
    PaginationInfo,
    homeworks::{
        entities::Homework,
        requests::HomeworkListQuery,
        responses::{HomeworkListResponse},
    },
};
use actix_web::HttpRequest;
use sqlx::Row;

pub async fn list_homeworks_with_pagination(
    storage: &SqliteStorage,
    user_id: i64,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {

    // 分页参数
    let page = query.pagination.page.max(1);
    let size = query.pagination.size.clamp(1, 100);
    let offset = (page - 1) * size;

    // 查询用户加入的班级
    let class_ids: Vec<i64> = sqlx::query_scalar(
        "SELECT class_id FROM class_users WHERE user_id = ?"
    )
        .bind(user_id)
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("Failed to fetch class IDs: {e}")))?;

    let mut items = vec![];
    let mut total = 0;

    if !class_ids.is_empty() {
        // 查询总数
        let query_total = format!(
            "SELECT COUNT(*) FROM homeworks WHERE class_id IN ({})",
            class_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );
        let mut total_query = sqlx::query_scalar::<_, i64>(&query_total);
        for id in &class_ids {
            total_query = total_query.bind(id);
        }
        total = total_query
            .fetch_one(&storage.pool)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("Failed to count homeworks: {e}")))?;

        // 查询分页数据
        if total > 0 {
            let query_list = format!(
                "SELECT * FROM homeworks WHERE class_id IN ({}) ORDER BY id DESC LIMIT ? OFFSET ?",
                class_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
            );
            let mut list_query = sqlx::query_as::<_, Homework>(&query_list);
            for id in &class_ids {
                list_query = list_query.bind(id);
            }
            list_query = list_query.bind(size).bind(offset);
            items = list_query
                .fetch_all(&storage.pool)
                .await
                .map_err(|e| HWSystemError::database_operation(format!("Failed to list homeworks: {e}")))?;
        }
    }

    let pages = if total == 0 { 0 } else { (total + size - 1) / size };

    Ok(HomeworkListResponse {
        items: items.into_iter().map(Homework::from).collect(),
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages,
        },
    })
}
