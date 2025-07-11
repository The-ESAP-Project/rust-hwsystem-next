use serde::{Deserialize, Serialize};

// 分页查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    #[serde(
        default = "default_page",
        deserialize_with = "deserialize_string_to_i64"
    )]
    pub page: i64,
    #[serde(
        default = "default_size",
        deserialize_with = "deserialize_string_to_i64"
    )]
    pub size: i64,
}

// 分页响应信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub size: i64,
    pub total: i64,
    pub pages: i64,
}

// 分页列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationInfo,
}

// 自定义反序列化函数，支持字符串到i64的转换
fn deserialize_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected, Visitor};
    use std::fmt;

    struct I64Visitor;

    impl<'de> Visitor<'de> for I64Visitor {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer or a string containing an integer")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if value <= i64::MAX as u64 {
                Ok(value as i64)
            } else {
                Err(Error::invalid_value(Unexpected::Unsigned(value), &self))
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            value
                .parse()
                .map_err(|_| Error::invalid_value(Unexpected::Str(value), &self))
        }
    }

    deserializer.deserialize_any(I64Visitor)
}

fn default_page() -> i64 {
    1
}

fn default_size() -> i64 {
    10
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self { page: 1, size: 10 }
    }
}
