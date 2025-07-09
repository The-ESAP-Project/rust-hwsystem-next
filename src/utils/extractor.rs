use actix_web::{Error as ActixError, FromRequest, HttpRequest, HttpResponse, dev::Payload};
use futures::future::{Ready, ready};

pub struct UserId(pub i64);

impl FromRequest for UserId {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let id_str = req.match_info().get("id").unwrap_or("");
        match id_str.parse::<i64>() {
            Ok(id) => ready(Ok(UserId(id))),
            Err(_) => {
                let resp = crate::api_models::common::response::ApiResponse::<()>::error_empty(
                    crate::api_models::ErrorCode::BadRequest,
                    "User ID format error, please provide a valid numeric ID.",
                );
                ready(Err(actix_web::error::InternalError::from_response(
                    "Invalid User ID",
                    HttpResponse::BadRequest().json(resp),
                )
                .into()))
            }
        }
    }
}
