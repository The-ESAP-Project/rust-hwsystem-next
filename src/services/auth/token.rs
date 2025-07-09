use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::api_models::ApiResponse;
use crate::middlewares::require_jwt::RequireJWT;
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
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response, "令牌刷新成功")))
                }
                Err(e) => {
                    tracing::error!("刷新令牌失败: {}", e);

                    // 清除无效的 refresh token cookie
                    let empty_cookie = jwt::JwtUtils::create_empty_refresh_token_cookie();

                    Ok(HttpResponse::Unauthorized().cookie(empty_cookie).json(
                        ApiResponse::error_empty(
                            crate::api_models::ErrorCode::Unauthorized,
                            "令牌已过期，请重新登录",
                        ),
                    ))
                }
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            crate::api_models::ErrorCode::Unauthorized,
            "未找到刷新令牌",
        ))),
    }
}

pub async fn handle_verify_token(
    _service: &AuthService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 从 Authorization header 中提取 token
    match RequireJWT::extract_access_token(request) {
        Some(token) => {
            if jwt::JwtUtils::verify_token(&token).is_ok() {
                Ok(HttpResponse::Ok().json(ApiResponse::success_empty("Token is valid")))
            } else {
                Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                    crate::api_models::ErrorCode::Unauthorized,
                    "令牌无效",
                )))
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            crate::api_models::ErrorCode::Unauthorized,
            "未提供令牌",
        ))),
    }
}

pub async fn handle_get_user(
    service: &AuthService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 从 Authorization header 中提取 token
    match RequireJWT::extract_user_id(request) {
        Some(user_id) => {
            // 这里可以实现获取用户信息的逻辑
            // 例如从数据库或缓存中获取用户信息
            let storage = service.get_storage(request);
            match storage.get_user_by_id(user_id).await {
                Ok(Some(user)) => {
                    Ok(HttpResponse::Ok().json(ApiResponse::success(user, "用户信息获取成功")))
                }
                Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                    crate::api_models::ErrorCode::NotFound,
                    "用户不存在",
                ))),
                Err(e) => {
                    tracing::error!("获取用户信息失败: {}", e);
                    Ok(
                        HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                            crate::api_models::ErrorCode::InternalServerError,
                            "获取用户信息失败",
                        )),
                    )
                }
            }
        }
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            crate::api_models::ErrorCode::Unauthorized,
            "未提供令牌",
        ))),
    }
}
