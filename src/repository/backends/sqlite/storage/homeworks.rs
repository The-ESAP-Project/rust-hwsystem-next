use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::common::pagination::PaginationInfo;
use crate::models::homeworks::{
    entities::Homework, requests::HomeworkListQuery, responses::HomeworkListResponse,
    responses::HomeworkResponse,
};
use sqlx::Row;

// pub async fn create_homework() 

pub async fn list_homeworks_with_pagination(
    storage: &SqliteStorage,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    unimplemented!()
}
