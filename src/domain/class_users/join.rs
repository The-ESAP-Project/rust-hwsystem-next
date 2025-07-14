use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use tracing::error;

use super::ClassUserService;
use crate::{
    middlewares::RequireJWT,
    models::{
        ApiResponse, ErrorCode,
        class_users::{entities::ClassUserRole, requests::JoinClassRequest},
    },
};

pub async fn join_class(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
    join_data: JoinClassRequest,
) -> ActixResult<HttpResponse> {
    let user_id = match RequireJWT::extract_user_id(request) {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user id",
            )));
        }
    };

    let storage = service.get_storage(request);
    let invite_code = &join_data.invite_code;

    let (class, user_student) = match storage
        .get_class_and_class_student_by_class_id_and_code(class_id, invite_code, user_id)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("Error getting class and user role by id and code: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::ClassJoinFailed,
                    "Failed to get class and user role",
                )),
            );
        }
    };

    tracing::debug!(class = ?class, user_student = ?user_student);

    if class.is_none() {
        return Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassInviteCodeInvalid,
            "Class not found or invite code is invalid",
        )));
    }
    if user_student.is_some() {
        return Ok(HttpResponse::Conflict().json(ApiResponse::error(
            ErrorCode::ClassAlreadyJoined,
            class.unwrap(),
            "User has already joined the class",
        )));
    }

    match storage
        .join_class(user_id, class_id, ClassUserRole::Student)
        .await
    {
        Ok(class_student) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class_student,
            "Class joined successfully",
        ))),
        Err(e) => {
            error!("Error joining class: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::ClassJoinFailed,
                    "Failed to join class",
                )),
            )
        }
    }
}
