use super::PostgresqlStorage;
use crate::errors::Result;
use crate::models::homeworks::{requests::HomeworkListQuery, responses::HomeworkListResponse};

pub async fn list_homeworks_with_pagination(
    storage: &PostgresqlStorage,
    query: HomeworkListQuery,
) -> Result<HomeworkListResponse> {
    unimplemented!()
}
