use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::models::{ApiResponse, ErrorCode, classes::requests::UpdateClassRequest};

pub async fn update_class(
    service: &ClassService,
    request: &HttpRequest,
    class_id: i64,
    update_data: UpdateClassRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.update_class(class_id, update_data).await {
        Ok(Some(class)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class,
            "Class information updated successfully",
        ))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassNotFound,
            "Class not found",
        ))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
            ErrorCode::ClassUpdateFailed,
            format!("Failed to update class information: {e}"),
        ))),
    }
}
