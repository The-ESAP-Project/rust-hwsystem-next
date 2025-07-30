// src/domain/homeworks/list.rs
use super::HomeworkService;
use crate::middlewares::require_jwt::RequireJWT;
use crate::models::users::entities::UserRole;
use crate::models::{ApiResponse, ErrorCode, homeworks::requests::HomeworkListQuery};
use actix_web::HttpRequest;
use actix_web::{HttpResponse, Result as ActixResult};

pub async fn list_homeworks(
    service: &HomeworkService,
    request: &HttpRequest,
    query: HomeworkListQuery,
) -> HttpResponse {
    let claims = match RequireJWT::extract_user_claims(request) {
        Some(claims) => claims,
        None => {
            return HttpResponse::Unauthorized().json(ApiResponse::error_empty(
                ErrorCode::Unauthorized,
                "Unauthorized access",
            ));
        }
    };

    let storage = service.get_storage(request);
    match storage
        .list_homeworks_with_pagination(claims.id, claims.role, query)
        .await
    {
        Ok(resp) => {
            HttpResponse::Ok().json(ApiResponse::success(resp, "Get homework list successfully"))
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::error_empty(
            ErrorCode::InternalServerError,
            format!("Failed to get homework list: {e}"),
        )),
    }
}
