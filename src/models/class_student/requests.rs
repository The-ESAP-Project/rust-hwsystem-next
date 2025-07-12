use serde::Deserialize;

// 加入班级请求
#[derive(Debug, Deserialize)]
pub struct JoinClassRequest {
    pub invite_code: String,
}
