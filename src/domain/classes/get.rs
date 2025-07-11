use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::models::{ApiResponse, ErrorCode};

pub async fn get_class(
    service: &ClassService,
    request: &HttpRequest,
    class_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.get_class_by_id(class_id).await {
        Ok(Some(class)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class,
            "Class information retrieved successfully",
        ))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassNotFound,
            "Class not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("Failed to get class information: {e}"),
            )),
        ),
    }
}
