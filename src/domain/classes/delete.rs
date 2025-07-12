use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::models::{ApiResponse, ErrorCode};

pub async fn delete_class(
    service: &ClassService,
    request: &HttpRequest,
    class_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.delete_class(class_id).await {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success_empty("Class deleted successfully")))
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassNotFound,
            "Class not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::ClassDeleteFailed,
                format!("Class deletion failed: {e}"),
            )),
        ),
    }
}
