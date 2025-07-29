// src/domain/homeworks/mod.rs
pub mod list;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::homeworks::requests::HomeworkListQuery;
use crate::repository::Storage;

pub struct HomeworkService {
    storage: Option<Arc<dyn Storage>>,
}

impl HomeworkService {
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

    // 列出作业
    pub async fn list_homeworks(
        &self,
        req: &HttpRequest,
        query: HomeworkListQuery,
    ) -> ActixResult<HttpResponse> {
        Ok(list::list_homeworks(self, req, query).await)
    }

    // TODO: 后续可以添加其他作业相关的方法
    // - create_homework
    // - get_homework
    // - update_homework
    // - delete_homework
}