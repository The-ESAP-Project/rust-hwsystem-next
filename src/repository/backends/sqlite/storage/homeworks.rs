use super::SqliteStorage;
use crate::errors::Result;
use crate::models::homeworks::{requests::HomeworkListQuery, responses::HomeworkListResponse};

// pub async fn create_homework()

pub async fn list_homeworks_with_pagination(
    storage: &SqliteStorage,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    unimplemented!()
}
