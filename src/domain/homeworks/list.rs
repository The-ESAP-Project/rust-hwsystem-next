use crate::api_models::{ApiResponse, ErrorCode, homeworks::requests::HomeworkListQuery};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::HomeworkService;

pub async fn list_homeworks(
    service: &HomeworkService,
    request: &HttpRequest,
    query: HomeworkListQuery,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.list_homeworks_with_pagination(query).await {
        Ok(resp) => Ok(HttpResponse::Ok().json(ApiResponse::success(resp, "获取作业列表成功"))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("获取作业列表失败: {e}"),
            )),
        ),
    }
}
