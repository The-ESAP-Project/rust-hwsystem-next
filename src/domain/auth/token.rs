use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::middlewares::require_jwt::RequireJWT;
use crate::models::{ApiResponse, ErrorCode};
use crate::utils::jwt;

use super::AuthService;

pub async fn handle_refresh_token(
    service: &AuthService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let config = service.get_config();
    // 从 cookie 中提取 refresh token
    match jwt::JwtUtils::extract_refresh_token_from_cookie(request) {
        Some(refresh_token) => {
            // 验证 refresh token 并生成新的 access token
            match jwt::JwtUtils::refresh_access_token(&refresh_token) {
                Ok(new_access_token) => {
                    let response = serde_json::json!({
                        "access_token": new_access_token,
                        "expires_in": config.jwt.access_token_expiry * 60, // 转换为秒
                        "token_type": "Bearer"
                    });
                    Ok(HttpResponse::Ok().json(ApiResponse::success(
                        response,
                        "Token refreshed successfully",
                    )))
                }
                Err(e) => {
                    tracing::error!("Refresh token failed: {}", e);

                    // 清除无效的 refresh token cookie
                    let empty_cookie = jwt::JwtUtils::create_empty_refresh_token_cookie();

                    Ok(HttpResponse::Unauthorized().cookie(empty_cookie).json(
                        ApiResponse::error_empty(
                            ErrorCode::Unauthorized,
                            "Login expired or invalid, please login again",
                        ),
                    ))
                }
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            ErrorCode::Unauthorized,
            "Unauthorized access, please login",
        ))),
    }
}

pub async fn handle_verify_token(
    _service: &AuthService,
    _request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success_empty("Token is valid")))
}

pub async fn handle_get_user(
    service: &AuthService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 从 Authorization header 中提取 token
    match RequireJWT::extract_user_id(request) {
        Some(user_id) => {
            // 从数据库中获取用户信息
            let storage = service.get_storage(request);
            match storage.get_user_by_id(user_id).await {
                Ok(Some(user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
                    user,
                    "User info retrieved successfully",
                ))),
                Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                    ErrorCode::UserNotFound,
                    "User not found",
                ))),
                Err(e) => {
                    tracing::error!("Get user info failed: {}", e);
                    Ok(
                        HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                            ErrorCode::InternalServerError,
                            "Get user info failed",
                        )),
                    )
                }
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            ErrorCode::Unauthorized,
            "Unauthorized access, please login",
        ))),
    }
}
