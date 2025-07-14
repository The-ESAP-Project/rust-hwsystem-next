use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::{
    domain::ClassStudentService,
    models::{ApiResponse, ErrorCode},
};

pub async fn list_class_students(
    service: &ClassStudentService,
    request: &HttpRequest,
    class_id: i64,
) -> ActixResult<HttpResponse> {
    Ok(
        HttpResponse::NotImplemented().json(ApiResponse::error_empty(
            ErrorCode::NotImplemented,
            "List class students not implemented",
        )),
    )
}
