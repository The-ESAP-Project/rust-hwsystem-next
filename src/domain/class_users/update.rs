use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::{
    domain::ClassUserService,
    models::{ApiResponse, ErrorCode, class_users::requests::UpdateStudentRequest},
};

pub async fn update_student(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
    class_student_id: i64,
    update_data: UpdateStudentRequest,
) -> ActixResult<HttpResponse> {
    Ok(
        HttpResponse::NotImplemented().json(ApiResponse::error_empty(
            ErrorCode::NotImplemented,
            "Update student not implemented",
        )),
    )
}
