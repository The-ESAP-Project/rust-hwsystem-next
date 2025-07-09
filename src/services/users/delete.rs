use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::api_models::{ApiResponse, ErrorCode};

pub async fn delete_user(
    service: &UserService,
    user_id: i64,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.delete_user(user_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::success_empty("用户删除成功"))),
        Ok(false) => Ok(HttpResponse::NotFound()
            .json(ApiResponse::error_empty(ErrorCode::NotFound, "用户不存在"))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("用户删除失败: {e}"),
            )),
        ),
    }
}
