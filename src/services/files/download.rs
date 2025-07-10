use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, http::header};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::FileService;
use crate::api_models::{ApiResponse, ErrorCode};
use crate::system::app_config::AppConfig;

pub async fn handle_download(
    service: &FileService,
    request: &HttpRequest,
    file_id: i64,
) -> ActixResult<HttpResponse> {
    let storage = service.get_storage(request);

    let db_file = match storage.get_file_by_id(file_id).await {
        Ok(Some(f)) => f,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(
                ErrorCode::FileNotFound,
                "File not found",
            )));
        }
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    format!("File query failed: {e}"),
                )),
            );
        }
    };

    let config = AppConfig::get();
    let upload_dir = &config.upload.dir;
    let file_path = format!("{}/{}.bin", upload_dir, db_file.unique_name);

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
                    "File open failed",
                )),
            );
        }
    };

    let mut buf = Vec::new();
    if file.read_to_end(&mut buf).is_err() {
        return Ok(
            HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                ErrorCode::InternalServerError,
                "File read failed",
            )),
        );
    }

    // 使用数据库中的原始文件名
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
        .insert_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", db_file.file_name),
        ))
        .body(buf))
}
