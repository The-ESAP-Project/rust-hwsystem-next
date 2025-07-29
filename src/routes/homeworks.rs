use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use once_cell::sync::Lazy;

use crate::models::homeworks::requests::HomeworkListQuery;
use crate::domain::homeworks::HomeworkService;
use crate::middlewares;
use crate::middlewares::require_jwt::RequireJWT;

// 懒加载的全局 HOMEWORK_SERVICE 实例
static HOMEWORK_SERVICE: Lazy<HomeworkService> = Lazy::new(HomeworkService::new_lazy);

async fn get_homeworks(
    request: HttpRequest,
    query: web::Query<HomeworkListQuery>,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE.list_homeworks(&request, query.into_inner()).await
}

pub fn configure_homeworks_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(middlewares::RequireJWT)
            .route("/homeworks", web::get().to(get_homeworks))
    );
}