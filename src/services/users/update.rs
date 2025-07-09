use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::api_models::{ApiResponse, ErrorCode, users::requests::UpdateUserRequest};

pub async fn update_user(
    service: &UserService,
    user_id: i64,
    update_data: UpdateUserRequest,
    request: Option<&HttpRequest>,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.update_user(user_id, update_data).await {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(user, "用户信息更新成功")))
        }
        Ok(None) => Ok(HttpResponse::NotFound()
            .json(ApiResponse::error_empty(ErrorCode::NotFound, "用户不存在"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::BadRequest,
            format!("用户信息更新失败: {e}"),
        ))),
    }
}
