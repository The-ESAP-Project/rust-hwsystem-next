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
use crate::api_models::{ApiResponse, files::requests::FileUploadForm};
use crate::system::app_config::AppConfig;

pub async fn handle_upload(
    _service: &FileService,
    _req: &HttpRequest,
    mut payload: Multipart,
) -> ActixResult<HttpResponse> {
    let mut form = FileUploadForm {
        file: String::new(),
        _type: String::new(),
        related_id: None,
    };

    let config = AppConfig::get();
    let upload_dir = &config.upload.dir;
    let max_size = config.upload.max_size;
    let allowed_types = &config.upload.allowed_types;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();

        let name = content_disposition
            .and_then(|cd| cd.get_name())
            .unwrap_or_default()
            .to_string();

        match name.as_str() {
            "file" => {
                // 获取文件类型
                let content_type = field
                    .content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_default();
                // 校验类型
                if !allowed_types.iter().any(|t| content_type.contains(t)) {
                    return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                        ErrorCode::FileTypeNotAllowed,
                        "文件类型不被允许",
                    )));
                }

                if !Path::new(upload_dir).exists() {
                    fs::create_dir_all(upload_dir)?;
                }
                let filename = format!("{}/{}.bin", upload_dir, Uuid::new_v4());
                let mut f = File::create(&filename)?;
                let mut total_size: usize = 0;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    total_size += data.len();
                    // 校验大小
                    if total_size > max_size {
                        // 删除已写入的文件
                        let _ = fs::remove_file(&filename);
                        return Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                            ErrorCode::FileSizeExceeded,
                            "文件大小超出限制",
                        )));
                    }
                    f.write_all(&data)?;
                }
                form.file = filename;
            }
            "type" => {
                let mut buffer = Vec::new();
                while let Some(chunk) = field.next().await {
                    buffer.extend_from_slice(&chunk?);
                }
                form._type = String::from_utf8(buffer).unwrap_or_default();
            }
            "related_id" => {
                let mut buffer = Vec::new();
                while let Some(chunk) = field.next().await {
                    buffer.extend_from_slice(&chunk?);
                }
                let s = String::from_utf8(buffer).unwrap_or_default();
                form.related_id = s.parse().ok();
            }
            _ => {}
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(form, "上传成功")))
}
