pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::classes::requests::{ClassQueryParams, CreateClassRequest, UpdateClassRequest};
use crate::repository::Storage;

pub struct ClassService {
    storage: Option<Arc<dyn Storage>>,
}

impl ClassService {
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

    // 获取班级列表
    pub async fn list_classes(
        &self,
        request: &HttpRequest,
        query: ClassQueryParams,
    ) -> ActixResult<HttpResponse> {
        list::list_classes(self, request, query).await
    }

    pub async fn create_class(
        &self,
        request: &HttpRequest,
        class_data: CreateClassRequest,
    ) -> ActixResult<HttpResponse> {
        create::create_class(self, request, class_data).await
    }

    // 根据班级 ID 获取班级信息
    pub async fn get_class(&self, request: &HttpRequest, class_id: i64) -> ActixResult<HttpResponse> {
        get::get_class(self, request, class_id).await
    }

    // 根据班级邀请码获取班级信息
    pub async fn get_class_by_code(
        &self,
        request: &HttpRequest,
        code: String,
    ) -> ActixResult<HttpResponse> {
        get::get_class_by_code(self, request, code).await
    }

    // 更新班级信息
    pub async fn update_class(
        &self,
        request: &HttpRequest,
        class_id: i64,
        update_data: UpdateClassRequest,
    ) -> ActixResult<HttpResponse> {
        update::update_class(self, request, class_id, update_data).await
    }

    // 根据班级 ID 删除班级
    pub async fn delete_class(
        &self,
        request: &HttpRequest,
        class_id: i64,
    ) -> ActixResult<HttpResponse> {
        delete::delete_class(self, request, class_id).await
    }
}
