use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::{
    middlewares::RequireJWT,
    models::{
        ApiResponse, ErrorCode,
        classes::requests::{ClassListQuery, ClassQueryParams},
        users::entities::UserRole,
    },
    storages::Storage,
};

pub async fn list_classes(
    service: &ClassService,
    request: &HttpRequest,
    query: ClassQueryParams,
) -> ActixResult<HttpResponse> {
    let role = RequireJWT::extract_user_role(request);
    let storage = service.get_storage(request);

    let uid = match RequireJWT::extract_user_id(request) {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing user id",
            )));
        }
    };

    let mut list_query = ClassListQuery {
        page: Some(query.pagination.page),
        size: Some(query.pagination.size),
        teacher_id: None,
        search: query.search,
    };

    // 权限校验
    if let Err(resp) = check_class_list_permission(role, uid, &mut list_query, &storage).await {
        return Ok(resp);
    }

    let result = storage.list_classes_with_pagination(list_query).await;
    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            response,
            "Class list retrieved successfully",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("Failed to retrieve class list: {e}"),
            )),
        ),
    }
}

/// 权限校验辅助函数
async fn check_class_list_permission(
    role: Option<UserRole>,
    uid: i64,
    query: &mut ClassListQuery,
    storage: &Arc<dyn Storage>,
) -> Result<(), HttpResponse> {
    match role {
        Some(UserRole::Admin) => {
            // 管理员可查全部班级
            // fallthrough
        }
        Some(UserRole::Teacher) => {
            // 教师只能查询自己的班级
            query.teacher_id = Some(uid);
        }
        Some(UserRole::User) => {
            // 学生只能查询自己的班级
            let result = storage
                .list_user_classes_with_pagination(uid, query.clone())
                .await;
            return match result {
                Ok(response) => Err(HttpResponse::Ok().json(ApiResponse::success(
                    response,
                    "User class list retrieved successfully",
                ))),
                Err(e) => Err(
                    HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                        ErrorCode::InternalServerError,
                        format!("Failed to retrieve user class list: {e}"),
                    )),
                ),
            };
        }
        _ => {
            return Err(HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized: missing required role",
            )));
        }
    }
    Ok(())
}
