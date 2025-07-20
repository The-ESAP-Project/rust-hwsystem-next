pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::models::homeworks::requests::HomeworkListQuery;
use crate::repository::Storage;
use crate::system::app_config::AppConfig;

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

    pub(crate) fn get_config(&self) -> &AppConfig {
        AppConfig::get()
    }

    pub async fn create_homework(
        &self,
        request: &HttpRequest,
        body: create::CreateHomeworkRequest,
    ) -> ActixResult<HttpResponse> {
        create::create_homework(self, request, body).await
    }

    pub async fn get_homework(
        &self,
        request: &HttpRequest,
        homework_id: i64,
    ) -> ActixResult<HttpResponse> {
        get::get_homework(self, request, homework_id).await
    }

    pub async fn update_homework(
        &self,
        request: &HttpRequest,
        homework_id: i64,
        body: update::UpdateHomeworkRequest,
    ) -> ActixResult<HttpResponse> {
        update::update_homework(self, request, homework_id, body).await
    }

    pub async fn delete_homework(
        &self,
        request: &HttpRequest,
        homework_id: i64,
    ) -> ActixResult<HttpResponse> {
        delete::delete_homework(self, request, homework_id).await
    }

    pub async fn list_homeworks(
        &self,
        request: &HttpRequest,
        query: HomeworkListQuery,
    ) -> ActixResult<HttpResponse> {
        list::list_homeworks(self, request, query).await
    }
}
