use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::sync::Arc;

use crate::api_models::{
    ApiResponse, ErrorCode,
    auth::{LoginRequest, LoginResponse},
    users::requests::CreateUserRequest,
};
use crate::storages::Storage;

pub struct AuthService {
    storage: Arc<dyn Storage>,
}

impl AuthService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }

    // 登录验证
    pub async fn login(&self, login_request: LoginRequest) -> ActixResult<HttpResponse> {
        // 1. 根据用户名或邮箱获取用户信息
        match self
            .storage
            .get_user_by_username_or_email(&login_request.username)
            .await
        {
            Ok(Some(user)) => {
                // 2. 验证密码
                if self.verify_password(&login_request.password, &user.password_hash) {
                    // 3. 更新最后登录时间
                    let _ = self.storage.update_last_login(user.id).await;

                    let response = LoginResponse {
                        access_token: user.generate_access_token(),
                        expires_in: 3600, // 假设令牌有效期为1小时
                        user,
                        created_at: chrono::Utc::now(),
                    };
                    Ok(HttpResponse::Ok().json(ApiResponse::success(response, "登录成功")))
                } else {
                    Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                        ErrorCode::Unauthorized,
                        "用户名或密码错误",
                    )))
                }
            }
            Ok(None) => Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
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

    pub async fn refresh_token(&self, _request: HttpRequest) -> ActixResult<HttpResponse> {
        // 刷新令牌逻辑尚未实现
        Ok(HttpResponse::NotImplemented().json("刷新令牌功能尚未实现"))
    }

    // 验证密码
    fn verify_password(&self, password: &str, hash: &str) -> bool {
        match PasswordHash::new(hash) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        }
    }

    // 生成密码哈希（用于注册新用户）
    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    // 用户注册
    pub async fn register(
        &self,
        mut create_request: CreateUserRequest,
    ) -> ActixResult<HttpResponse> {
        // 1. 检查用户名是否已存在
        match self
            .storage
            .get_user_by_username(&create_request.username)
            .await
        {
            Ok(Some(_)) => {
                return Ok(HttpResponse::Conflict().json(ApiResponse::error_empty(
                    ErrorCode::Conflict,
                    "用户名已存在",
                )));
            }
            Ok(None) => {
                // 用户名不存在，继续检查邮箱
            }
            Err(e) => {
                return Ok(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::InternalServerError,
                        format!("注册失败: {e}"),
                    )),
                );
            }
        }

        // 2. 检查邮箱是否已存在
        match self.storage.get_user_by_email(&create_request.email).await {
            Ok(Some(_)) => {
                return Ok(HttpResponse::Conflict()
                    .json(ApiResponse::error_empty(ErrorCode::Conflict, "邮箱已存在")));
            }
            Ok(None) => {
                // 邮箱不存在，可以继续注册
            }
            Err(e) => {
                return Ok(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::InternalServerError,
                        format!("注册失败: {e}"),
                    )),
                );
            }
        }

        // 3. 哈希密码
        match self.hash_password(&create_request.password) {
            Ok(password_hash) => {
                // 将明文密码替换为哈希后的密码
                create_request.password = password_hash;

                // 4. 创建用户
                match self.storage.create_user(create_request).await {
                    Ok(user) => {
                        Ok(HttpResponse::Created().json(ApiResponse::success(user, "注册成功")))
                    }
                    Err(e) => Ok(HttpResponse::InternalServerError().json(
                        ApiResponse::error_empty(
                            ErrorCode::InternalServerError,
                            format!("注册失败: {e}"),
                        ),
                    )),
                }
            }
            Err(e) => Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    format!("密码哈希失败: {e}"),
                )),
            ),
        }
    }
}
