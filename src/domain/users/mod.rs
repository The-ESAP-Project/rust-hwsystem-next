pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::api_models::users::requests::{CreateUserRequest, UpdateUserRequest, UserQueryParams};
use crate::storages::Storage;

pub struct UserService {
    storage: Option<Arc<dyn Storage>>,
}

impl UserService {
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

    // 获取用户列表
    pub async fn list_users(
        &self,
        query: UserQueryParams,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        list::list_users(self, query, request).await
    }

    // 创建用户
    pub async fn create_user(
        &self,
        user_data: CreateUserRequest,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        create::create_user(self, user_data, request).await
    }

    // 根据ID获取用户
    pub async fn get_user(&self, user_id: i64, request: &HttpRequest) -> ActixResult<HttpResponse> {
        get::get_user(self, user_id, request).await
    }

    // 更新用户信息
    pub async fn update_user(
        &self,
        user_id: i64,
        update_data: UpdateUserRequest,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        update::update_user(self, user_id, update_data, request).await
    }

    // 删除用户
    pub async fn delete_user(
        &self,
        user_id: i64,
        request: &HttpRequest,
    ) -> ActixResult<HttpResponse> {
        delete::delete_user(self, user_id, request).await
    }
}
