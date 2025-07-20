#[macro_export]
macro_rules! define_safe_i64_extractor {
    ($name:ident, $key:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
        pub struct $name(pub i64);

        impl actix_web::FromRequest for $name {
            type Error = actix_web::Error;
            type Future = std::future::Ready<Result<Self, Self::Error>>;

            fn from_request(
                request: &actix_web::HttpRequest,
                _: &mut actix_web::dev::Payload,
            ) -> Self::Future {
                use actix_web::{HttpResponse, error};
                let id_str = request.match_info().get($key).unwrap_or("");
                match id_str.parse::<i64>() {
                    Ok(id) => std::future::ready(Ok(Self(id))),
                    Err(_) => {
                        let resp = $crate::models::common::response::ApiResponse::<()>::error_empty(
                            $crate::models::ErrorCode::BadRequest,
                            concat!($key, " format error, please provide a valid numeric ID."),
                        );
                        std::future::ready(Err(error::InternalError::from_response(
                            "Invalid ID",
                            HttpResponse::BadRequest().json(resp),
                        )
                        .into()))
                    }
                }
            }
        }
    };
}

define_safe_i64_extractor!(SafeIDI64, "id");
define_safe_i64_extractor!(SafeClassIdI64, "class_id");
