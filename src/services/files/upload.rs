use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};

use super::FileService;
use crate::api_models::ApiResponse;

pub async fn handle_upload(
    service: &FileService,
    request: &HttpRequest,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    Ok(HttpResponse::Ok().json(ApiResponse::success_empty("File uploaded successfully")))
}
