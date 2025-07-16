use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::middlewares::require_jwt::RequireJWT;
use crate::models::auth::responses::{
    RefreshTokenResponse, TokenVerificationResponse, UserInfoResponse,
};
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
                    let response = RefreshTokenResponse {
                        access_token: new_access_token,
                        expires_in: config.jwt.access_token_expiry,
                    };
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
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        TokenVerificationResponse { is_valid: true },
        "Token is valid",
    )))
}

pub async fn handle_get_user(
    _service: &AuthService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 从 Authorization header 中提取 token
    match RequireJWT::extract_user_claims(request) {
        Some(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            UserInfoResponse { user },
            "User information retrieved successfully",
        ))),
        None => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            ErrorCode::Unauthorized,
            "Unauthorized access, please login",
        ))),
    }
}
