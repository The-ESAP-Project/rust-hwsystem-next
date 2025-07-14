use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use tracing::error;

use crate::{
    domain::ClassUserService,
    models::{
        ApiResponse, ErrorCode,
        class_users::requests::{ClassUserQuery, ClassUserQueryParams},
    },
};

pub async fn list_class_users_with_pagination(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
    query: ClassUserQueryParams,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    let list_query = ClassUserQuery {
        page: Some(query.pagination.page),
        size: Some(query.pagination.size),
        search: query.search,
    };

    match storage
        .list_class_users_with_pagination(class_id, list_query)
        .await
    {
        Ok(class_users) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class_users,
            "Class users retrieved successfully",
        ))),
        Err(err) => {
            error!("Failed to retrieve class users: {}", err);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    "Failed to retrieve class users",
                )),
            )
        }
    }
}
