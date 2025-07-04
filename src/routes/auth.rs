use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use std::sync::Arc;

use crate::api_models::auth::requests::LoginRequest;
use crate::api_models::users::requests::CreateUserRequest;
use crate::services::AuthService;
use crate::storages::Storage;

pub async fn login(
    storage: web::Data<Arc<dyn Storage>>,
    user_data: web::Json<LoginRequest>,
) -> ActixResult<HttpResponse> {
    let service = AuthService::new(storage.get_ref().clone());
    service.login(user_data.into_inner()).await
}

pub async fn refresh_token(request: HttpRequest) -> ActixResult<HttpResponse> {
    let service = AuthService::new(
        request
            .app_data::<web::Data<Arc<dyn Storage>>>()
            .unwrap()
            .get_ref()
            .clone(),
    );
    service.refresh_token(request).await
}

pub async fn register(
    storage: web::Data<Arc<dyn Storage>>,
    user_data: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    let service = AuthService::new(storage.get_ref().clone());
    service.register(user_data.into_inner()).await
}

// 配置路由
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh_token))
            .route("/register", web::post().to(register)),
    );
}
