pub mod list;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::api_models::homeworks::requests::HomeworkListQuery;
use crate::storages::Storage;
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

    pub async fn list_homeworks(
        &self,
        request: &HttpRequest,
        query: HomeworkListQuery,
    ) -> ActixResult<HttpResponse> {
        list::list_homeworks(self, request, query).await
    }
}
