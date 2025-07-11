pub mod settings;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::system::app_config::AppConfig;

pub struct SystemService;

impl SystemService {
    pub fn new_lazy() -> Self {
        Self
    }

    pub(crate) fn get_config(&self) -> &AppConfig {
        AppConfig::get()
    }

    // Handle file upload
    pub async fn get_settings(&self, request: &HttpRequest) -> ActixResult<HttpResponse> {
        settings::get_settings(self, request).await
    }
}
