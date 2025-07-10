use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use regex::Regex;
use tracing::error;

use super::UserService;
use crate::api_models::{ApiResponse, ErrorCode, users::requests::CreateUserRequest};
use crate::utils::password::hash_password;

pub async fn create_user(
    service: &UserService,
    mut user_data: CreateUserRequest,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 用户名长度校验：5 <= x <= 16
    if user_data.username.len() < 5 || user_data.username.len() > 16 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::BadRequest,
            "Username length must be between 5 and 16 characters",
        )));
    }

    // 用户名校验：只允许英文、-、_
    let username_re = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    if !username_re.is_match(&user_data.username) {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::BadRequest,
            "Username must contain only letters, numbers, underscores or hyphens",
        )));
    }

    user_data.password = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                ErrorCode::BadRequest,
                format!("Password hashing failed: {e}"),
            )));
        }
    };

    let storage = service.get_storage(request);

    match storage.create_user(user_data).await {
        Ok(user) => Ok(HttpResponse::Created().json(ApiResponse::success(user, "用户创建成功"))),
        Err(e) => {
            let msg = format!("User creation failed: {e}");
            error!("{}", msg);
            // 判断是否唯一约束冲突
            if msg.contains("UNIQUE constraint failed") {
                Ok(HttpResponse::Conflict().json(ApiResponse::error_empty(
                    ErrorCode::Conflict,
                    "Username or email already exists",
                )))
            } else {
                Ok(HttpResponse::BadRequest()
                    .json(ApiResponse::error_empty(ErrorCode::BadRequest, msg)))
            }
        }
    }
}
