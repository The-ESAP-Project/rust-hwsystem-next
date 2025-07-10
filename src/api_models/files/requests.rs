use serde::Serialize;

#[derive(PartialEq, Serialize)]
pub struct FileUploadForm {
    /// 文件内容
    pub file: String,

    /// 文件类型
    #[serde(rename = "type")]
    pub _type: String,

    /// 关联的提交ID
    #[serde(rename = "related_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_id: Option<isize>,
}
