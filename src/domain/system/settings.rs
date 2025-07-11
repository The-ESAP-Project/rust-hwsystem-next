use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::SystemService;
use crate::models::ApiResponse;

pub async fn get_settings(
    service: &SystemService,
    _req: &HttpRequest,
) -> ActixResult<HttpResponse> {
    // 获取配置
    let config = service.get_config();

    // 构建响应
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        config.clone(),
        "Settings retrieved successfully",
    )))
}
