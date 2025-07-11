use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::HomeworkService;
use crate::middlewares;
use crate::models::homeworks::requests::HomeworkListQuery;

// 懒加载的全局 HomeworkService 实例
static HOMEWORK_SERVICE: Lazy<HomeworkService> = Lazy::new(HomeworkService::new_lazy);

// HTTP处理程序
pub async fn list_homeworks(
    req: HttpRequest,
    query: web::Query<HomeworkListQuery>,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE
        .list_homeworks(&req, query.into_inner())
        .await
}

// 配置路由
pub fn configure_homeworks_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/homeworks")
            .wrap(middlewares::RequireJWT)
            .route("", web::get().to(list_homeworks)),
    );
}
