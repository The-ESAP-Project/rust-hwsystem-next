use super::PostgresqlStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::common::pagination::PaginationInfo;
use crate::models::homeworks::{
    entities::Homework, requests::HomeworkListQuery, responses::HomeworkListResponse,
};
use chrono::{TimeZone, Utc};
use serde_json::json;
use sqlx::Row;

pub async fn list_homeworks_with_pagination(
    storage: &PostgresqlStorage,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    unimplemented!()
}
