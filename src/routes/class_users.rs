use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassUserService;
use crate::middlewares;
use crate::models::class_users::entities::ClassUserRole;
use crate::models::class_users::requests::{JoinClassRequest, UpdateStudentRequest};
use crate::models::users::entities::UserRole;
use crate::utils::SafeIDI64;

use crate::define_safe_i64_extractor;

// 用于从请求路径中安全地提取 class_student_id
define_safe_i64_extractor!(SafeClassUserID, "class_student_id");

// 懒加载的全局 CLASS_STUDENT_SERVICE 实例
static CLASS_STUDENT_SERVICE: Lazy<ClassUserService> = Lazy::new(ClassUserService::new_lazy);

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

pub async fn get_class_student(
    req: HttpRequest,
    path: SafeClassUserID,
) -> ActixResult<HttpResponse> {
    // let class_student_id = path.0;
    // CLASS_STUDENT_SERVICE
    //     .get_class_student(&req, class_student_id)
    //     .await
    unimplemented!("get_class_student not implemented yet");
}

pub async fn list_class_users(req: HttpRequest, path: SafeIDI64) -> ActixResult<HttpResponse> {
    CLASS_STUDENT_SERVICE.list_class_users(&req, path.0).await
}

pub async fn update_student(
    req: HttpRequest,
    path: web::Path<(SafeIDI64, SafeClassUserID)>,
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
pub fn configure_class_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/classes/{id}/students")
            .wrap(middlewares::RequireJWT)
            .service(
                web::resource("")
                    // 学生加入班级，需要传入班级 ID 和邀请码，User 权限
                    .route(web::post().to(join_class))
                    .wrap(middlewares::RequireRole::new(&UserRole::User)),
            )
            .service(
                web::resource("")
                    // 列出班级学生，Class_Representative 或更高权限
                    .route(web::get().to(list_class_users))
                    .wrap(middlewares::RequireClassRole::new_any(
                        ClassUserRole::class_representative_roles(),
                    )),
            )
            .service(
                web::resource("/{class_student_id}")
                    .route(web::get().to(get_class_student))
                    .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
            )
            .service(
                web::resource("/{class_student_id}")
                    .route(web::put().to(update_student))
                    .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
            ),
    );
}
