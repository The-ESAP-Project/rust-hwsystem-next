use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::ClassService;
use crate::models::{
    ApiResponse, ErrorCode,
    classes::requests::{ClassListQuery, ClassQueryParams},
};

pub async fn list_classes(
    service: &ClassService,
    request: &HttpRequest,
    query: ClassQueryParams,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    let list_query = ClassListQuery {
        page: Some(query.pagination.page),
        size: Some(query.pagination.size),
        search: query.search,
    };

    match storage.list_classes_with_pagination(list_query).await {
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
