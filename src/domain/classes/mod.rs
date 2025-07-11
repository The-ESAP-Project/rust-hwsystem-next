use actix_web::HttpRequest;
use std::sync::Arc;

use crate::storages::Storage;
use crate::system::app_config::AppConfig;

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

    pub(crate) fn get_config(&self) -> &AppConfig {
        AppConfig::get()
    }
}
