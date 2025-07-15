use crate::{
    domain::ClassUserService,
    middlewares::{RequireClassRole, RequireJWT},
    models::{
        ApiResponse, ErrorCode,
        class_users::entities::{ClassUser, ClassUserRole},
        users::entities::{User, UserRole},
    },
};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

pub async fn get_class_user(
    service: &ClassUserService,
    req: &HttpRequest,
    class_id: i64,
    class_user_id: i64,
) -> ActixResult<HttpResponse> {
    let user_claims = match RequireJWT::extract_user_claims(req) {
        Some(claims) => claims,
        None => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user claims",
            )));
        }
    };

    let class_user = RequireClassRole::extract_user_class_user(req);

    // 权限校验
    if let Err(resp) =
        check_class_user_get_permission(&user_claims, &class_user, class_id, class_user_id)
    {
        return Ok(resp);
    }

    let storage = service.get_storage(req);

    // 查询目标班级用户信息
    let target_user = if let Some(ref cu) = class_user {
        // 自己
        if cu.user_id == user_claims.id {
            Some(cu.clone())
        } else {
            // 教师/班长查其他人
            match storage
                .get_class_user_by_user_id_and_class_id(class_id, class_user_id)
                .await
            {
                Ok(Some(class_user)) => Some(class_user),
                Ok(None) => {
                    return Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                        ErrorCode::NotFound,
                        "Class user not found",
                    )));
                }
                Err(e) => {
                    return Ok(
                        HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                            ErrorCode::InternalServerError,
                            format!("Internal error: {e}"),
                        )),
                    );
                }
            }
        }
    } else {
        // 管理员直接查
        match storage
            .get_class_user_by_user_id_and_class_id(class_id, class_user_id)
            .await
        {
            Ok(Some(class_user)) => Some(class_user),
            Ok(None) => {
                return Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                    ErrorCode::NotFound,
                    "Class user not found",
                )));
            }
            Err(e) => {
                return Ok(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::InternalServerError,
                        format!("Internal error: {e}"),
                    )),
                );
            }
        }
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        target_user,
        "Class user retrieved successfully",
    )))
}

fn check_class_user_get_permission(
    user_claims: &User,
    class_user: &Option<ClassUser>,
    class_id: i64,
    class_user_id: i64,
) -> Result<(), HttpResponse> {
    // 管理员直接放行
    if user_claims.role == UserRole::Admin {
        return Ok(());
    }

    match class_user {
        Some(class_user) => {
            if class_user.class_id != class_id {
                return Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
                    ErrorCode::ClassPermissionDenied,
                    "Class ID mismatch",
                )));
            }
            match class_user.role {
                ClassUserRole::Teacher | ClassUserRole::ClassRepresentative => Ok(()),
                ClassUserRole::Student => {
                    if class_user.id == class_user_id {
                        Ok(())
                    } else {
                        Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
                            ErrorCode::ClassPermissionDenied,
                            "You do not have permission to access this resource",
                        )))
                    }
                }
            }
        }
        None => Err(HttpResponse::Forbidden().json(ApiResponse::error_empty(
            ErrorCode::ClassPermissionDenied,
            "You do not have permission to access this resource",
        ))),
    }
}
