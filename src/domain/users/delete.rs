use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::{
    middlewares::RequireJWT,
    models::{ApiResponse, ErrorCode},
};

pub async fn delete_user(
    service: &UserService,
    user_id: i64,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    if let Some(current_user_id) = RequireJWT::extract_user_id(request) {
        if user_id == current_user_id || user_id == 1 {
            // 禁止删除超级管理员用户和当前用户
            return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                ErrorCode::CanNotDeleteCurrentUser,
                "Cannot delete current user",
            )));
        }
    }

    match storage.delete_user(user_id).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success_empty("User deleted successfully")))
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::UserNotFound,
            "User not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::UserDeleteFailed,
                format!("User deletion failed: {e}"),
            )),
        ),
    }
}
