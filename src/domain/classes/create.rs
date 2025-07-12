use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::models::ApiResponse;
use crate::models::classes::requests::CreateClassRequest;

// TODO: Implement the actual class creation logic
pub async fn create_class(
    _service: &ClassService,
    _request: &HttpRequest,
    _class_data: CreateClassRequest,
) -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Created().json(ApiResponse::success_empty("班级创建成功")))
}
