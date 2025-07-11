use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassService;
use crate::middlewares;
use crate::utils::SafeI64;

// 懒加载的全局 CLASS_SERVICE 实例
static CLASS_SERVICE: Lazy<ClassService> = Lazy::new(ClassService::new_lazy);

pub async fn get_class(req: HttpRequest, id: SafeI64) -> ActixResult<HttpResponse> {
    CLASS_SERVICE.get_class(&req, id.0).await
}

// 配置路由
pub fn configure_classes_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/classes")
            .wrap(middlewares::RequireJWT)
            .route("/{id}", web::get().to(get_class)),
    );
}
