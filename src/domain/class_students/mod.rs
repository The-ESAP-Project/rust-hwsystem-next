pub mod join;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::class_student::requests::JoinClassRequest;
use crate::storages::Storage;

pub struct ClassStudentService {
    storage: Option<Arc<dyn Storage>>,
}

impl ClassStudentService {
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
}
