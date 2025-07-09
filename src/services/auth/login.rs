use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::api_models::{
    ApiResponse, ErrorCode,
    auth::{LoginRequest, LoginResponse},
};
use crate::cache::traits::TypedObjectCache;
use crate::utils::jwt;
use crate::utils::password::verify_password;

use super::AuthService;

pub async fn handle_login(
    service: &AuthService,
    login_request: LoginRequest,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);
    let cache = service.get_cache(request);
    let config = service.get_config();

    // 1. 根据用户名或邮箱获取用户信息
    match storage
        .get_user_by_username_or_email(&login_request.username)
        .await
    {
        Ok(Some(user)) => {
            // 2. 验证密码
            if verify_password(&login_request.password, &user.password_hash) {
                // 3. 更新最后登录时间
                let _ = storage.update_last_login(user.id).await;

                // 4. 生成令牌对
                match user
                    .generate_token_pair(login_request.remember_me.then(|| {
                        chrono::Duration::days(config.jwt.refresh_token_remember_me_expiry)
                    }))
                    .await
                {
                    Ok(token_pair) => {
                        // 生成 Access Token 和 Refresh Token 成功
                        tracing::info!("用户 {} 登录成功", user.username);

                        let response = LoginResponse {
                            access_token: token_pair.access_token,
                            expires_in: config.jwt.access_token_expiry * 60, // 转换为秒
                            user,
                            created_at: chrono::Utc::now(),
                        };

                        // 5. 缓存 AccessToken (15分钟)
                        cache
                            .insert(
                                response.access_token.clone(),
                                response.user.clone(),
                                (config.jwt.access_token_expiry * 60) as u32,
                            )
                            .await;

                        // 6. 创建 refresh token cookie
                        let refresh_cookie =
                            jwt::JwtUtils::create_refresh_token_cookie(&token_pair.refresh_token);

                        Ok(HttpResponse::Ok()
                            .cookie(refresh_cookie)
                            .json(ApiResponse::success(response, "登录成功")))
                    }
                    Err(e) => {
                        tracing::error!("生成 JWT 令牌失败: {}", e);
                        Ok(
                            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                                ErrorCode::InternalServerError,
                                "登录失败，无法生成令牌",
                            )),
                        )
                    }
                }
            } else {
                Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                    ErrorCode::AuthFailed,
                    "用户名或密码错误",
                )))
            }
        }
        Ok(None) => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
            ErrorCode::AuthFailed,
            "用户名或密码错误",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("登录失败: {e}"),
            )),
        ),
    }
}
