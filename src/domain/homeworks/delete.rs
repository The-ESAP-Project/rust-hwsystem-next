use crate::models::{ApiResponse, ErrorCode, homeworks::requests::HomeworkListQuery};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::HomeworkService;

pub async fn delete_homework(
    service: &HomeworkService,
    request: &HttpRequest,
    query: HomeworkListQuery,
) -> ActixResult<HttpResponse> {
    unimplemented!()
}
