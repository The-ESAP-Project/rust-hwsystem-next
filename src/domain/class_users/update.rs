use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::{
    domain::ClassUserService,
    middlewares::RequireJWT,
    models::{
        ApiResponse, ErrorCode,
        class_users::requests::UpdateClassUserRequest,
        classes::entities::Class,
        users::entities::{User, UserRole},
    },
};

pub async fn update_class_user(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
    class_user_id: i64,
    update_data: UpdateClassUserRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    let user = match RequireJWT::extract_user_claims(request) {
        Some(user) => user,
        _ => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user claims",
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
    if let Err(resp) = check_update_class_user_permissions(&user, &class) {
        return Ok(resp);
    }

    match storage
        .update_class_user(class_id, class_user_id, update_data)
        .await
    {
        Ok(Some(class_user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class_user,
            "Class user updated successfully",
        ))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassUserNotFound,
            "Class user not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("Failed to get class user: {e}"),
            )),
        ),
    }
}

fn check_update_class_user_permissions(user: &User, class: &Class) -> Result<(), HttpResponse> {
    match user.role {
        UserRole::Admin => Ok(()),
        UserRole::Teacher if class.teacher_id == user.id => Ok(()),
        _ => Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
            ErrorCode::ClassPermissionDenied,
            "You do not have permission to update class users",
        ))),
    }
}
