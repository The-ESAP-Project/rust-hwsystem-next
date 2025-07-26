use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HomeworkListQuery {
    #[serde(flatten)]
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub status: Option<String>,
}
