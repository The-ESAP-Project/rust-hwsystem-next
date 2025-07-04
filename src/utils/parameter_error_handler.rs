use actix_web::HttpResponse;
use actix_web::error::{JsonPayloadError, QueryPayloadError};

// 通用的错误处理器 - 处理JSON和查询参数错误
pub fn json_error_handler(
    err: JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    create_error_response(format_json_error(&err))
}

pub fn query_error_handler(
    err: QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    create_error_response(format_query_error(&err))
}

// 格式化JSON错误信息
fn format_json_error(err: &JsonPayloadError) -> String {
    match err {
        JsonPayloadError::Deserialize(e) => {
            let error_str = e.to_string();
            if error_str.contains("无效的用户角色") || error_str.contains("无效的用户状态")
            {
                error_str
            } else if error_str.contains("unknown variant") {
                format!("无效的枚举值: {e}")
            } else {
                format!("JSON格式错误: {e}")
            }
        }
        JsonPayloadError::ContentType => "请求Content-Type必须为application/json".to_string(),
        JsonPayloadError::Payload(e) => format!("请求体错误: {e}"),
        _ => "请求数据格式错误".to_string(),
    }
}

// 格式化查询参数错误信息
fn format_query_error(err: &QueryPayloadError) -> String {
    match err {
        QueryPayloadError::Deserialize(e) => {
            let error_str = e.to_string();
            if error_str.contains("无效的用户角色") || error_str.contains("无效的用户状态")
            {
                error_str
            } else if error_str.contains("unknown variant") {
                format!("无效的查询参数枚举值: {e}")
            } else {
                format!("查询参数格式错误: {e}")
            }
        }
        _ => "查询参数错误".to_string(),
    }
}

// 创建统一的错误响应
fn create_error_response(error_msg: String) -> actix_web::Error {
    use crate::api_models::ErrorCode;
    use crate::api_models::common::response::ApiResponse;

    let response = ApiResponse::<()>::error_empty(ErrorCode::BadRequest, &error_msg);
    let json_response = HttpResponse::BadRequest().json(response);
    actix_web::error::InternalError::from_response(error_msg, json_response).into()
}
