use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, Result as ActixResult};
use futures_util::TryStreamExt;
use futures_util::stream::StreamExt;
use std::fs;
use std::io::Write;
use std::{fs::File, path::Path};
use uuid::Uuid;

use super::FileService;
use crate::api_models::ErrorCode;
use crate::api_models::{ApiResponse, files::responses::FileUploadResponse};
use crate::middlewares::RequireJWT;
use crate::system::app_config::AppConfig;

pub async fn handle_upload(
    service: &FileService,
    req: &HttpRequest,
    mut payload: Multipart,
) -> ActixResult<HttpResponse> {
    // 获取配置
    let config = AppConfig::get();
    let upload_dir = &config.upload.dir;
    let max_size = config.upload.max_size;
    let allowed_types = &config.upload.allowed_types;

    // 确保上传目录存在
    if !Path::new(upload_dir).exists() {
        fs::create_dir_all(upload_dir).map_err(actix_web::error::ErrorInternalServerError)?;
    }

    // 文件相关信息
    let mut unique_name = String::new();
    let mut original_filename = String::new();
    let mut file_size: i64 = 0;
    let mut file_type = String::new();
    let mut file_uploaded = false; // 新增变量

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let name = content_disposition
            .and_then(|cd| cd.get_name())
            .unwrap_or_default()
            .to_string();

        if name == "file" {
            if file_uploaded {
                return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                    ErrorCode::MuitifileUploadNotAllowed,
                    "Only one file can be uploaded at a time",
                )));
            }
            file_uploaded = true;
            // 获取文件类型
            let content_type = field
                .content_type()
                .map(|ct| ct.to_string())
                .unwrap_or_default();
            // 校验类型
            if !allowed_types.iter().any(|t| content_type.contains(t)) {
                return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                    ErrorCode::FileTypeNotAllowed,
                    "File type not allowed",
                )));
            }

            // 获取原始文件名
            original_filename = content_disposition
                .and_then(|cd| cd.get_filename())
                .map(|s| s.to_string())
                .unwrap_or_default();

            unique_name = format!("{}{}", chrono::Utc::now().timestamp(), Uuid::new_v4());
            let file_path = format!("{upload_dir}/{unique_name}.bin");
            let mut f = File::create(&file_path)?;
            let mut total_size: usize = 0;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                total_size += data.len();
                // 校验大小
                if total_size > max_size {
                    let _ = fs::remove_file(&file_path);
                    return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                        ErrorCode::FileSizeExceeded,
                        "File size exceeds the limit",
                    )));
                }
                f.write_all(&data)?;
            }
            file_size = total_size as i64;
            file_type = content_type;
        }
    }

    let storage = service.get_storage(req);

    let user_id = RequireJWT::extract_user_id(req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not authenticated"))?;

    let db_file = match storage
        .upload_file(
            &unique_name,
            &original_filename,
            &file_size,
            &file_type,
            user_id,
        )
        .await
    {
        Ok(file) => FileUploadResponse {
            id: file.id,
            filename: file.unique_name,
            original_name: file.file_name,
            size: file.file_size,
            content_type: file.file_type,
            uploaded_at: file.uploaded_at,
        },
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    ErrorCode::InternalServerError,
                    format!("Failed to upload file: {e}"),
                )),
            );
        }
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(db_file, "File uploaded successfully")))
}
