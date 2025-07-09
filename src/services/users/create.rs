use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::api_models::{ApiResponse, ErrorCode, users::requests::CreateUserRequest};

pub async fn create_user(
    service: &UserService,
    user_data: CreateUserRequest,
    request: Option<&HttpRequest>,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.create_user(user_data).await {
        Ok(user) => Ok(HttpResponse::Created().json(ApiResponse::success(user, "用户创建成功"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::BadRequest,
            format!("用户创建失败: {e}"),
        ))),
    }
}
