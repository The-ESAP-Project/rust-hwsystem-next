use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassStudentService;
use crate::middlewares;
use crate::models::class_student::requests::JoinClassRequest;
use crate::models::users::entities::UserRole;
use crate::models::{ApiResponse, ErrorCode};
use crate::utils::SafeI64;

// 懒加载的全局 CLASS_SERVICE 实例
static CLASS_SERVICE: Lazy<ClassStudentService> = Lazy::new(ClassStudentService::new_lazy);

// HTTP处理程序
pub async fn join_class(
    req: HttpRequest,
    path: SafeI64,
    join_data: web::Json<JoinClassRequest>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0;
    CLASS_SERVICE
        .join_class(&req, class_id, join_data.into_inner())
        .await
}

pub async fn update_student(
    req: HttpRequest,
    path: web::Path<(SafeI64, SafeI64)>,
    // update_data: web::Json<UpdateStudentRequest>,
) -> ActixResult<HttpResponse> {
    Ok(
        HttpResponse::NotImplemented().json(ApiResponse::error_empty(
            ErrorCode::NotFound,
            "未实现的功能",
        )),
    )
}

// 配置路由
// TODO: 路由需要进一步优化，并且允许当前创建的用户访问自己的资源
pub fn configure_class_students_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/classes/{id}/students")
            .wrap(middlewares::RequireJWT)
            .service(
                web::resource("")
                    .route(web::post().to(join_class))
                    .wrap(middlewares::RequireRole::new(UserRole::USER)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::put().to(update_student))
                    .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
            ),
    );
}
