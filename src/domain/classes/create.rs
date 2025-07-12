use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use tracing::error;

use super::ClassService;
use crate::models::classes::requests::CreateClassRequest;
use crate::models::{ApiResponse, ErrorCode};

pub async fn create_class(
    service: &ClassService,
    request: &HttpRequest,
    class_data: CreateClassRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.create_class(class_data).await {
        Ok(user) => Ok(HttpResponse::Created().json(ApiResponse::success(user, "班级创建成功"))),
        Err(e) => {
            let msg = format!("Class creation failed: {e}");
            error!("{}", msg);
            // 判断是否唯一约束冲突
            if msg.contains("UNIQUE constraint failed") {
                Ok(HttpResponse::Conflict().json(ApiResponse::error_empty(
                    ErrorCode::ClassAlreadyExists,
                    "Classname already exists",
                )))
            } else if msg.contains("FOREIGN KEY constraint failed") {
                Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                    ErrorCode::ClassCreationFailed,
                    "Teacher does not exist",
                )))
            } else {
                Ok(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::ClassCreationFailed,
                        msg,
                    )),
                )
            }
        }
    }
}
