use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, http::header};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::FileService;
use crate::api_models::{ApiResponse, ErrorCode};
use crate::system::app_config::AppConfig;

pub async fn handle_download(
    _service: &FileService,
    _request: &HttpRequest,
    file_id: String,
) -> ActixResult<HttpResponse> {
    let config = AppConfig::get();
    let upload_dir = &config.upload.dir;
    let file_path = format!("{upload_dir}/{file_id}.bin");

    if !Path::new(&file_path).exists() {
        return Ok(HttpResponse::NotFound()
            .json(ApiResponse::error_empty(ErrorCode::NotFound, "文件不存在")));
    }

    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(_) => {
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    "文件读取失败",
                )),
            );
        }
    };

    let mut buf = Vec::new();
    if file.read_to_end(&mut buf).is_err() {
        return Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                "文件读取失败",
            )),
        );
    }

    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
        .insert_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{file_id}.bin\""),
        ))
        .body(buf))
}
