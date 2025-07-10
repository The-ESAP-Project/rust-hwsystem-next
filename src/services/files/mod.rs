pub mod download;
pub mod upload;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::storages::Storage;

pub struct FileService {
    storage: Option<Arc<dyn Storage>>,
}

impl FileService {
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

    // Handle file upload
    pub async fn handle_upload(&self, request: &HttpRequest) -> ActixResult<HttpResponse> {
        upload::handle_upload(self, request).await
    }

    // Handle file download
    pub async fn handle_download(
        &self,
        request: &HttpRequest,
        file_id: String,
    ) -> ActixResult<HttpResponse> {
        download::handle_download(self, request, file_id).await
    }
}
