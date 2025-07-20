use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::HomeworkService;
use crate::middlewares;
use crate::models::homeworks::requests::HomeworkListQuery;

// 懒加载的全局 HomeworkService 实例
static HOMEWORK_SERVICE: Lazy<HomeworkService> = Lazy::new(HomeworkService::new_lazy);

// HTTP处理程序
pub async fn create_homework(
    request: &HttpRequest,
    body: create::CreateHomeworkRequest,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE.create_homework(self, request, body).await
}

pub async fn get_homework(
    request: &HttpRequest,
    homework_id: i64,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE.get_homework(self, request, homework_id).await
}

pub async fn update_homework(
    request: &HttpRequest,
    homework_id: i64,
    body: update::UpdateHomeworkRequest,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE.update_homework(self, request, homework_id, body).await
}

pub async fn delete_homework(
    request: &HttpRequest,
    homework_id: i64,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE.delete_homework(self, request, homework_id).await
}

pub async fn list_homeworks(
    request: HttpRequest,
    query: web::Query<HomeworkListQuery>,
) -> ActixResult<HttpResponse> {
    HOMEWORK_SERVICE
        .list_homeworks(&request, query.into_inner())
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
