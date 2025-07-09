use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::UserService;
use crate::api_models::{
    ApiResponse, ErrorCode,
    users::requests::{UserListQuery, UserQueryParams},
};

pub async fn list_users(
    service: &UserService,
    query: UserQueryParams,
    request: Option<&HttpRequest>,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    let list_query = UserListQuery {
        page: Some(query.pagination.page),
        size: Some(query.pagination.size),
        role: query.role,
        status: query.status,
        search: query.search,
    };

    match storage.list_users_with_pagination(list_query).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(ApiResponse::success(response, "获取用户列表成功")))
        }
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("获取用户列表失败: {e}"),
            )),
        ),
    }
}
