pub mod delete;
pub mod get;
pub mod join;
pub mod list;
pub mod update;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::class_users::requests::{
    ClassUserListParams, JoinClassRequest, UpdateClassUserRequest,
};
use crate::repository::Storage;

pub struct ClassUserService {
    storage: Option<Arc<dyn Storage>>,
}

impl ClassUserService {
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

    // 加入班级
    pub async fn join_class(
        &self,
        request: &HttpRequest,
        class_id: i64,
        join_data: JoinClassRequest,
    ) -> ActixResult<HttpResponse> {
        join::join_class(self, request, class_id, join_data).await
    }

    // 列出班级用户
    pub async fn list_class_users_with_pagination(
        &self,
        request: &HttpRequest,
        class_id: i64,
        query: ClassUserListParams,
    ) -> ActixResult<HttpResponse> {
        list::list_class_users_with_pagination(self, request, class_id, query).await
    }

    // 获取班级用户信息
    pub async fn get_class_user(
        &self,
        request: &HttpRequest,
        class_id: i64,
        class_user_id: i64,
    ) -> ActixResult<HttpResponse> {
        get::get_class_user(self, request, class_id, class_user_id).await
    }

    // 更新用户信息
    pub async fn update_class_user(
        &self,
        request: &HttpRequest,
        class_id: i64,
        class_user_id: i64,
        update_data: UpdateClassUserRequest,
    ) -> ActixResult<HttpResponse> {
        update::update_class_user(self, request, class_id, class_user_id, update_data).await
    }

    // 删除用户
    pub async fn delete_class_user(
        &self,
        request: &HttpRequest,
        class_id: i64,
        class_user_id: i64,
    ) -> ActixResult<HttpResponse> {
        delete::delete_class_user(self, request, class_id, class_user_id).await
    }
}
