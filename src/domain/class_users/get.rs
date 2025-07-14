use crate::{
    domain::ClassUserService,
    models::{ApiResponse, ErrorCode},
};
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

pub async fn get_class_student(
    service: &ClassUserService,
    req: &HttpRequest,
    class_id: i64,
    class_user_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(req);

    match storage
        .get_class_user_by_user_id_and_class_id(class_user_id, class_id)
        .await
    {
        Ok(Some(class_user)) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            class_user,
            "Class user information retrieved successfully",
        ))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
            ErrorCode::ClassUserNotFound,
            "Class user not found",
        ))),
        Err(e) => Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                format!("Failed to get class user information: {e}"),
            )),
        ),
    }
}
