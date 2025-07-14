use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use crate::{
    domain::ClassUserService,
    models::{ApiResponse, ErrorCode, class_users::requests::UpdateClassUserRequest},
};

pub async fn update_user(
    service: &ClassUserService,
    request: &HttpRequest,
    class_id: i64,
    user_id: i64,
    update_data: UpdateClassUserRequest,
) -> ActixResult<HttpResponse> {
    Ok(
        HttpResponse::NotImplemented().json(ApiResponse::error_empty(
            ErrorCode::NotImplemented,
            "Update class user not implemented",
        )),
    )
}
