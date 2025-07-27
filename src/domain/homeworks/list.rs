use crate::models::{ApiResponse, ErrorCode, homeworks::requests::HomeworkListQuery};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use crate::middlewares::RequireJWT;
use super::HomeworkService;

pub async fn list_homeworks(
    service: &HomeworkService,
    request: &HttpRequest,
    query: HomeworkListQuery,
) -> ActixResult<HttpResponse> {
    let user_claims = match RequireJWT::extract_user_claims(request) {
        Some(claims) => claims,
        None => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user claims",
            )));
        }
    };
    let user_id = user_claims.id;
    let storage = service.get_storage(request);

    match storage.list_homeworks_with_pagination(user_id, query).await {
        Ok(resp) => Ok(HttpResponse::Ok().json(ApiResponse::success(resp, "获取作业列表成功"))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("获取作业列表失败: {e}"),
            )),
        ),
    }
}
