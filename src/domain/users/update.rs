use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::models::{
    ApiResponse, ErrorCode,
    users::{requests::UpdateUserRequest, responses::UserResponse},
};

pub async fn update_user(
    service: &UserService,
    user_id: i64,
    mut update_data: UpdateUserRequest,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    if let Some(password) = update_data.password {
        match crate::utils::password::hash_password(&password) {
            Ok(hash) => update_data.password = Some(hash),
            Err(e) => {
                return Ok(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::InternalServerError,
                        format!("Password hashing failed: {e}"),
                    )),
                );
            }
        }
    }

    match storage.update_user(user_id, update_data).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            UserResponse { user },
            "User information updated successfully",
        ))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::UserNotFound,
            "User not found",
        ))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::UserUpdateFailed,
            format!("Failed to update user information: {e}"),
        ))),
    }
}
