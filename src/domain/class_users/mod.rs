pub mod join;
pub mod list;
pub mod update;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::class_users::requests::{JoinClassRequest, UpdateStudentRequest};
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
        req: &HttpRequest,
        class_id: i64,
        join_data: JoinClassRequest,
    ) -> ActixResult<HttpResponse> {
        join::join_class(self, req, class_id, join_data).await
    }

    // 列出班级学生
    pub async fn list_class_users(
        &self,
        req: &HttpRequest,
        class_id: i64,
    ) -> ActixResult<HttpResponse> {
        list::list_class_users(self, req, class_id).await
    }

    // 更新学生信息
    pub async fn update_student(
        &self,
        req: &HttpRequest,
        class_id: i64,
        class_student_id: i64,
        update_data: UpdateStudentRequest,
    ) -> ActixResult<HttpResponse> {
        update::update_student(self, req, class_id, class_student_id, update_data).await
    }
}
