use crate::{
    domain::ClassUserService,
    middlewares::RequireJWT,
    models::{ApiResponse, ErrorCode, classes::entities::Class, users::entities::UserRole},
};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

pub async fn delete_class_user(
    service: &ClassUserService,
    req: &HttpRequest,
    class_id: i64,
    class_user_id: i64,
) -> ActixResult<HttpResponse> {
    let user_role = RequireJWT::extract_user_role(req);
    let storage = service.get_storage(req);

    let uid = match RequireJWT::extract_user_id(req) {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user id",
            )));
        }
    };

    // 查询班级信息
    let class = match storage.get_class_by_id(class_id).await {
        Ok(Some(class)) => class,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                ErrorCode::ClassNotFound,
                "Class not found",
            )));
        }
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    format!("Failed to get class information: {e}"),
                )),
            );
        }
    };

    // 权限校验
    if let Err(resp) = check_class_user_delete_permission(user_role, uid, class_user_id, &class) {
        return Ok(resp);
    }

    // 如果被删除者为本班教师，则禁止删除
    if class.teacher_id == class_user_id {
        return Ok(HttpResponse::Forbidden().json(ApiResponse::error_empty(
            ErrorCode::ClassPermissionDenied,
            "You cannot delete the class teacher. Please transfer or delete the class first.",
        )));
    }

    match storage.leave_class(class_user_id, class_id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::success_empty(
            "Class user deleted successfully",
        ))),
        Ok(false) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassUserNotFound,
            "Class user not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("Failed to delete class user: {e}"),
            )),
        ),
    }
}

/// 权限校验辅助函数
fn check_class_user_delete_permission(
    role: Option<UserRole>,
    uid: i64,
    class_user_id: i64,
    class: &Class,
) -> Result<(), HttpResponse> {
    match role {
        Some(UserRole::Admin) => Ok(()),
        Some(UserRole::Teacher) => {
            if class.teacher_id != uid {
                Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
                    ErrorCode::ClassPermissionDenied,
                    "You do not have permission to delete another teacher's class user",
                )))
            } else {
                Ok(())
            }
        }
        Some(UserRole::User) => {
            if class_user_id == uid {
                Ok(())
            } else {
                Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
                    ErrorCode::ClassPermissionDenied,
                    "You do not have permission to delete this class user",
                )))
            }
        }
        _ => Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
            ErrorCode::ClassPermissionDenied,
            "You do not have permission to delete this class user",
        ))),
    }
}
