pub mod login;
pub mod register;
pub mod token;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::cache::ObjectCache;
use crate::storages::Storage;

pub struct AuthService {
    storage: Option<Arc<dyn Storage>>,
    cache: Option<Arc<dyn ObjectCache>>,
}

impl AuthService {
    pub fn new_lazy() -> Self {
        Self {
            storage: None,
            cache: None,
        }
    }

    pub(crate) fn get_storage(&self, request: Option<&HttpRequest>) -> Arc<dyn Storage> {
        if let Some(storage) = &self.storage {
            storage.clone()
        } else if let Some(req) = request {
            req.app_data::<actix_web::web::Data<Arc<dyn Storage>>>()
                .expect("Storage not found in app data")
                .get_ref()
                .clone()
        } else {
            panic!("No storage available")
        }
    }

    pub(crate) fn get_cache(&self, request: Option<&HttpRequest>) -> Arc<dyn ObjectCache> {
        if let Some(cache) = &self.cache {
            cache.clone()
        } else if let Some(req) = request {
            req.app_data::<actix_web::web::Data<Arc<dyn ObjectCache>>>()
                .expect("Cache not found in app data")
                .get_ref()
                .clone()
        } else {
            panic!("No cache available")
        }
    }

    // 登录验证
    pub async fn login(
        &self,
        login_request: crate::api_models::auth::LoginRequest,
        request: Option<&HttpRequest>,
    ) -> ActixResult<HttpResponse> {
        login::handle_login(self, login_request, request).await
    }

    // 用户注册
    pub async fn register(
        &self,
        create_request: crate::api_models::users::requests::CreateUserRequest,
        request: Option<&HttpRequest>,
    ) -> ActixResult<HttpResponse> {
        register::handle_register(self, create_request, request).await
    }

    // 刷新令牌
    pub async fn refresh_token(&self, request: HttpRequest) -> ActixResult<HttpResponse> {
        token::handle_refresh_token(self, request).await
    }

    // 验证令牌
    pub async fn verify_token(&self, request: HttpRequest) -> ActixResult<HttpResponse> {
        token::handle_verify_token(self, request).await
    }
}
