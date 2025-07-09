use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::api_models::users::requests::{CreateUserRequest, UpdateUserRequest, UserQueryParams};
use crate::middlewares;
use crate::services::UserService;
use crate::utils::UserId;

// 懒加载的全局 UserService 实例
static USER_SERVICE: Lazy<UserService> = Lazy::new(UserService::new_lazy);

// HTTP处理程序
pub async fn list_users(
    req: HttpRequest,
    query: web::Query<UserQueryParams>,
) -> ActixResult<HttpResponse> {
    USER_SERVICE.list_users(query.into_inner(), &req).await
}

pub async fn create_user(
    req: HttpRequest,
    user_data: web::Json<CreateUserRequest>,
) -> ActixResult<HttpResponse> {
    USER_SERVICE.create_user(user_data.into_inner(), &req).await
}

pub async fn get_user(req: HttpRequest, user_id: UserId) -> ActixResult<HttpResponse> {
    USER_SERVICE.get_user(user_id.0, &req).await
}

pub async fn update_user(
    req: HttpRequest,
    user_id: UserId,
    update_data: web::Json<UpdateUserRequest>,
) -> ActixResult<HttpResponse> {
    USER_SERVICE
        .update_user(user_id.0, update_data.into_inner(), &req)
        .await
}

pub async fn delete_user(req: HttpRequest, user_id: UserId) -> ActixResult<HttpResponse> {
    USER_SERVICE.delete_user(user_id.0, &req).await
}

// 配置路由
pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .wrap(middlewares::RequireJWT)
            .service(
                web::scope("")
                    .wrap(middlewares::RequireRole::new("admin"))
                    .route("", web::get().to(list_users))
                    .route("", web::post().to(create_user))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user)),
            ),
    );
}
