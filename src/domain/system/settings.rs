use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::SystemService;
use crate::models::{ApiResponse, system::responses::SystemSettingsResponse};

pub async fn get_settings(
    service: &SystemService,
    _req: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 获取配置
    let config = service.get_config();

    let response = SystemSettingsResponse {
        system_name: config.app.system_name.clone(),
        max_file_size: config.upload.max_size as u64,
        allowed_file_types: config.upload.allowed_types.clone(),
        environment: config.app.environment.clone(),
        log_level: config.app.log_level.clone(),
    };

    // 构建响应
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        response,
        "Settings retrieved successfully",
    )))
}
