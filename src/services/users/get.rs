use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::api_models::{ApiResponse, ErrorCode};

pub async fn get_user(
    service: &UserService,
    user_id: i64,
    request: Option<&HttpRequest>,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(user, "获取用户信息成功")))
        }
        Ok(None) => Ok(HttpResponse::NotFound()
            .json(ApiResponse::error_empty(ErrorCode::NotFound, "用户不存在"))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("获取用户信息失败: {e}"),
            )),
        ),
    }
}
