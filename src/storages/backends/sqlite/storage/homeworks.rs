use sqlx::Row;
use super::SqliteStorage;
use crate::api_models::homeworks::{
    entities::Homework,
    responses::HomeworkResponse,
    responses::HomeworkListResponse,
    requests::HomeworkListQuery,
};
use crate::api_models::common::pagination::PaginationInfo;
use crate::errors::{HWSystemError, Result};
use chrono::{TimeZone, Utc};
use serde_json::json;

pub async fn list_homeworks_with_pagination(
    storage: &SqliteStorage,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    let page = query.pagination.page.max(1) as u32;
    let size = query.pagination.size.clamp(1, 100) as u32;
    let offset = (page - 1) * size;

    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(status) = &query.status {
        conditions.push("status = ?");
        params.push(status.clone());
    }
    if let Some(search) = &query.search {
        conditions.push("(title LIKE ? OR description LIKE ?)");
        let pattern = format!("%{}%", search);
        params.push(pattern.clone());
        params.push(pattern);
    }

    let where_clause = if conditions.is_empty() {
        "".to_string()
    } else {
        format!(" WHERE {}", conditions.join(" AND "))
    };

    let order_by = match query.order_by.as_deref() {
        Some("deadline") => "deadline",
        Some("title") => "title",
        _ => "created_at",
    };
    let order = match query.order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // 查询总数
    let count_sql = format!("SELECT COUNT(*) as total FROM homeworks{}", where_clause);
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }
    let total_row = count_query
        .fetch_one(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询作业总数失败: {e}")))?;
    let total: i64 = total_row.get("total");

    // 查询数据
    let data_sql = format!(
        "SELECT * FROM homeworks{} ORDER BY {} {} LIMIT ? OFFSET ?",
        where_clause, order_by, order
    );
    let mut data_query = sqlx::query_as::<_, Homework>(&data_sql);
    for param in &params {
        data_query = data_query.bind(param);
    }
    data_query = data_query.bind(size as i64).bind(offset as i64);

    let items: Vec<Homework> = data_query
        .fetch_all(&storage.pool)
        .await
        .map_err(|e| HWSystemError::database_operation(format!("查询作业列表失败: {e}")))?;

    // 类型转换，保证返回结构与接口定义一致
    let items: Vec<HomeworkResponse> = items.iter().map(homework_to_response).collect();

    let pages = ((total + size as i64 - 1) / size as i64) as u32;

    Ok(HomeworkListResponse {
        items,
        pagination: PaginationInfo {
            page: page as i64,
            size: size as i64,
            total: total as i64,
            pages: pages as i64,
        },
    })
}

fn homework_to_response(hw: &Homework) -> HomeworkResponse {
    HomeworkResponse {
        id: hw.id,
        title: hw.title.clone(),
        description: hw.description.clone(),
        content: hw.content.clone(),
        deadline: hw.deadline
            .map(|ts| Utc.timestamp_opt(ts, 0).unwrap().to_rfc3339())
            .unwrap_or_default(),
        max_score: hw.max_score,
        allow_late_submission: hw.allow_late_submission != 0,
        attachments: hw.attachments
            .as_ref()
            .and_then(|s| serde_json::from_str::<Vec<Option<String>>>(s).ok())
            .unwrap_or_default(),
        submission_count: hw.submission_count,
        status: hw.status.clone(),
        created_by: json!({ "id": hw.created_by }), // 可根据需要补充更多用户信息
        created_at: Utc.timestamp_opt(hw.created_at, 0).unwrap().to_rfc3339(),
        updated_at: Utc.timestamp_opt(hw.updated_at, 0).unwrap().to_rfc3339(),
    }
}