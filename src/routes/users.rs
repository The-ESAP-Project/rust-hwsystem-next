use actix_web::{HttpResponse, Result as ActixResult, web};
use std::sync::Arc;

use crate::api_models::users::requests::{CreateUserRequest, UpdateUserRequest, UserQueryParams};
use crate::services::UserService;
use crate::storages::Storage;

// HTTP处理程序
pub async fn list_users(
    storage: web::Data<Arc<dyn Storage>>,
    query: web::Query<UserQueryParams>,
) -> ActixResult<HttpResponse> {
    let service = UserService::new(storage.get_ref().clone());
    service.list_users(query.into_inner()).await
}

pub async fn create_user(
    storage: web::Data<Arc<dyn Storage>>,
    user_data: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    let service = UserService::new(storage.get_ref().clone());
    service.create_user(user_data.into_inner()).await
}

pub async fn get_user(
    storage: web::Data<Arc<dyn Storage>>,
    path: web::Path<i64>,
) -> ActixResult<HttpResponse> {
    let service = UserService::new(storage.get_ref().clone());
    service.get_user(path.into_inner()).await
}

pub async fn update_user(
    storage: web::Data<Arc<dyn Storage>>,
    path: web::Path<i64>,
    update_data: web::Json<UpdateUserRequest>,
) -> ActixResult<HttpResponse> {
    let service = UserService::new(storage.get_ref().clone());
    service
        .update_user(path.into_inner(), update_data.into_inner())
        .await
}

pub async fn delete_user(
    storage: web::Data<Arc<dyn Storage>>,
    path: web::Path<i64>,
) -> ActixResult<HttpResponse> {
    let service = UserService::new(storage.get_ref().clone());
    service.delete_user(path.into_inner()).await
}

// 配置路由
pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}
