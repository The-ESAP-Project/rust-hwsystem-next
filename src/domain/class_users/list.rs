use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use tracing::error;

use crate::{
    domain::ClassUserService,
    models::{ApiResponse, ErrorCode, classes::entities::Class, users::entities::UserRole},
};

pub async fn list_class_users(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    match storage.list_class_users(class_id).await {
        Ok(class_users) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class_users,
            "Class students retrieved successfully",
        ))),
        Err(err) => {
            error!("Failed to retrieve class students: {}", err);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    "Failed to retrieve class students",
                )),
            )
        }
    }
}

/// 权限校验辅助函数
async fn check_class_student_list_permission(
    role: Option<UserRole>,
    uid: i64,
    class: &Class,
) -> Result<(), HttpResponse> {
    match role {
        Some(UserRole::Admin) => Ok(()),
        Some(UserRole::Teacher) => {
            if class.teacher_id != uid {
                Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
                    ErrorCode::ClassPermissionDenied,
                    "You do not have permission to delete another teacher's class",
                )))
            } else {
                Ok(())
            }
        }
        _ => Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
            ErrorCode::ClassPermissionDenied,
            "You do not have permission to delete this class",
        ))),
    }
}
