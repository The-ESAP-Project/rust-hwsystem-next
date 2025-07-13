use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassService;
use crate::middlewares;
use crate::models::classes::requests::{ClassQueryParams, CreateClassRequest, UpdateClassRequest};
use crate::models::users::entities::UserRole;
use crate::utils::SafeIDI64;

// 懒加载的全局 CLASS_SERVICE 实例
static CLASS_SERVICE: Lazy<ClassService> = Lazy::new(ClassService::new_lazy);

// HTTP处理程序
pub async fn list_classes(
    req: HttpRequest,
    query: web::Query<ClassQueryParams>,
) -> ActixResult<HttpResponse> {
    CLASS_SERVICE.list_classes(&req, query.into_inner()).await
}

pub async fn create_class(
    req: HttpRequest,
    class_data: web::Json<CreateClassRequest>,
) -> ActixResult<HttpResponse> {
    CLASS_SERVICE
        .create_class(&req, class_data.into_inner())
        .await
}

pub async fn get_class(req: HttpRequest, id: SafeIDI64) -> ActixResult<HttpResponse> {
    CLASS_SERVICE.get_class(&req, id.0).await
}

pub async fn get_class_by_code(
    req: HttpRequest,
    code: web::Path<String>,
) -> ActixResult<HttpResponse> {
    CLASS_SERVICE
        .get_class_by_code(&req, code.into_inner())
        .await
}

pub async fn update_class(
    req: HttpRequest,
    id: SafeIDI64,
    update_data: web::Json<UpdateClassRequest>,
) -> ActixResult<HttpResponse> {
    CLASS_SERVICE
        .update_class(&req, id.0, update_data.into_inner())
        .await
}

pub async fn delete_class(req: HttpRequest, id: SafeIDI64) -> ActixResult<HttpResponse> {
    CLASS_SERVICE.delete_class(&req, id.0).await
}

// 配置路由
pub fn configure_classes_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/classes")
            .wrap(middlewares::RequireJWT)
            .service(
                // TODO: 路由需要进一步优化，并且允许当前创建的用户访问自己的资源
                web::resource("").route(web::get().to(list_classes)).route(
                    web::post()
                        .to(create_class)
                        // TODO: 路由需要进一步优化，并且允许当前创建的用户访问自己的资源
                        .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
                ),
            )
            .service(
                web::resource("/code/{code}").route(
                    web::get()
                        .to(get_class_by_code)
                        .wrap(middlewares::RequireRole::new(UserRole::USER)),
                ),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_class))
                    .route(
                        web::put()
                            .to(update_class)
                            .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
                    )
                    .route(
                        web::delete()
                            .to(delete_class)
                            .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
                    ),
            ),
    );
}
