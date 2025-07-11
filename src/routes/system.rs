use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, middleware, web};
use once_cell::sync::Lazy;

use crate::middlewares;
use crate::services::SystemService;

// 懒加载的全局 SystemService 实例
static SYSTEM_SERVICE: Lazy<SystemService> = Lazy::new(SystemService::new_lazy);

pub async fn get_settings(request: HttpRequest) -> ActixResult<HttpResponse> {
    SYSTEM_SERVICE.get_settings(&request).await
}

// 配置路由
pub fn configure_system_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/system")
            .wrap(middleware::Compress::default())
            .wrap(middlewares::RequireJWT)
            .service(
                web::scope("")
                    .wrap(middlewares::RequireRole::new("admin"))
                    .route("/settings", web::get().to(get_settings)),
            ),
    );
}
