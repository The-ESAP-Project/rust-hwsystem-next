use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassStudentService;
use crate::middlewares;
use crate::models::class_student::requests::{JoinClassRequest, UpdateStudentRequest};
use crate::models::users::entities::UserRole;
use crate::utils::SafeIDI64;

use crate::define_safe_i64_extractor;

// 用于从请求路径中安全地提取 class_student_id
define_safe_i64_extractor!(SafeClassStudentID, "class_student_id");

// 懒加载的全局 CLASS_STUDENT_SERVICE 实例
static CLASS_STUDENT_SERVICE: Lazy<ClassStudentService> = Lazy::new(ClassStudentService::new_lazy);

// HTTP处理程序
pub async fn join_class(
    req: HttpRequest,
    path: SafeIDI64,
    join_data: web::Json<JoinClassRequest>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0;
    CLASS_STUDENT_SERVICE
        .join_class(&req, class_id, join_data.into_inner())
        .await
}

pub async fn list_class_students(req: HttpRequest, path: SafeIDI64) -> ActixResult<HttpResponse> {
    CLASS_STUDENT_SERVICE
        .list_class_students(&req, path.0)
        .await
}

pub async fn update_student(
    req: HttpRequest,
    path: web::Path<(SafeIDI64, SafeClassStudentID)>,
    update_data: web::Json<UpdateStudentRequest>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0.0;
    let class_student_id = path.1.0;

    CLASS_STUDENT_SERVICE
        .update_student(&req, class_id, class_student_id, update_data.into_inner())
        .await
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
                    .wrap(middlewares::RequireRole::new(&UserRole::User)),
            )
            .service(web::resource("").route(web::get().to(list_class_students)))
            .service(
                web::resource("/{class_student_id}")
                    .route(web::put().to(update_student))
                    .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
            ),
    );
}
