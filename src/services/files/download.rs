
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::FileService;
use crate::api_models::ApiResponse;

pub async fn handle_download(
    service: &FileService,
    request: &HttpRequest,
    file_id: String,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    Ok(HttpResponse::Ok().json(ApiResponse::success_empty(
        format!("File downloaded successfully, {file_id}"),
    )))
}
