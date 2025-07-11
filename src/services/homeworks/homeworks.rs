use crate::api_models::{ApiResponse, ErrorCode, homeworks::requests::HomeworkListQuery};
use crate::storages::Storage;
use actix_web::{HttpResponse, Result as ActixResult, web};
use std::sync::Arc;

pub async fn list_homeworks(
    storage: web::Data<Arc<dyn Storage>>,
    query: web::Query<HomeworkListQuery>,
) -> ActixResult<HttpResponse> {
    match storage
        .list_homeworks_with_pagination(query.into_inner())
        .await
    {
        Ok(resp) => Ok(HttpResponse::Ok().json(ApiResponse::success(resp, "获取作业列表成功"))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("获取作业列表失败: {e}"),
            )),
        ),
    }
}
