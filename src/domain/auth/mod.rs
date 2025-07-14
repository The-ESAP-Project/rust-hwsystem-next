pub mod login;
pub mod register;
pub mod token;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::repository::Storage;
use crate::system::app_config::AppConfig;

pub struct AuthService {
    storage: Option<Arc<dyn Storage>>,
}

impl AuthService {
    pub fn new_lazy() -> Self {
        Self { storage: None }
    }

    pub(crate) fn get_storage(&self, request: &HttpRequest) -> Arc<dyn Storage> {
        if let Some(storage) = &self.storage {
            storage.clone()
        } else {
            request
                .app_data::<actix_web::web::Data<Arc<dyn Storage>>>()
                .expect("Storage not found in app data")
                .get_ref()
                .clone()
        }
    }

    pub(crate) fn get_config(&self) -> &AppConfig {
        AppConfig::get()
    }

    // 登录验证
    pub async fn login(
        &self,
        login_request: crate::models::auth::LoginRequest,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        login::handle_login(self, login_request, request).await
    }

    // 用户注册
    pub async fn register(
        &self,
        create_request: crate::models::users::requests::CreateUserRequest,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        register::handle_register(self, create_request, request).await
    }

    // 刷新令牌
    pub async fn refresh_token(&self, request: &HttpRequest) -> ActixResult<HttpResponse> {
        token::handle_refresh_token(self, request).await
    }

    // 验证令牌
    pub async fn verify_token(&self, request: &HttpRequest) -> ActixResult<HttpResponse> {
        token::handle_verify_token(self, request).await
    }

    // 获取用户信息
    pub async fn get_user(&self, request: &HttpRequest) -> ActixResult<HttpResponse> {
        // 这里可以实现获取用户信息的逻辑
        // 例如从数据库或缓存中获取用户信息
        token::handle_get_user(self, request).await
    }
}
