use actix_web::{HttpRequest, HttpResponse, Result as ActixResult, web};
use once_cell::sync::Lazy;

use crate::domain::ClassUserService;
use crate::middlewares;
use crate::models::class_users::entities::ClassUserRole;
use crate::models::class_users::requests::{
    ClassUserListParams, JoinClassRequest, UpdateClassUserRequest,
};
use crate::models::users::entities::UserRole;
use crate::utils::SafeClassIdI64;

use crate::define_safe_i64_extractor;

// 用于从请求路径中安全地提取 class_user_id
define_safe_i64_extractor!(SafeClassUserID, "class_user_id");

// 懒加载的全局 CLASS_STUDENT_SERVICE 实例
static CLASS_STUDENT_SERVICE: Lazy<ClassUserService> = Lazy::new(ClassUserService::new_lazy);

// HTTP处理程序
pub async fn join_class(
    request: HttpRequest,
    path: SafeClassIdI64,
    join_data: web::Json<JoinClassRequest>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0;
    CLASS_STUDENT_SERVICE
        .join_class(&request, class_id, join_data.into_inner())
        .await
}

pub async fn list_class_users_with_pagination(
    request: HttpRequest,
    path: SafeClassIdI64,
    query: web::Query<ClassUserListParams>,
) -> ActixResult<HttpResponse> {
    CLASS_STUDENT_SERVICE
        .list_class_users_with_pagination(&request, path.0, query.into_inner())
        .await
}

pub async fn get_class_user(
    request: HttpRequest,
    path: web::Path<(SafeClassIdI64, SafeClassUserID)>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0.0;
    let class_user_id = path.1.0;
    CLASS_STUDENT_SERVICE
        .get_class_user(&request, class_id, class_user_id)
        .await
}

pub async fn update_class_user(
    request: HttpRequest,
    path: web::Path<(SafeClassIdI64, SafeClassUserID)>,
    update_data: web::Json<UpdateClassUserRequest>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0.0;
    let class_user_id = path.1.0;

    CLASS_STUDENT_SERVICE
        .update_class_user(&request, class_id, class_user_id, update_data.into_inner())
        .await
}

pub async fn delete_class_user(
    request: HttpRequest,
    path: web::Path<(SafeClassIdI64, SafeClassUserID)>,
) -> ActixResult<HttpResponse> {
    let class_id = path.0.0;
    let class_user_id = path.1.0;
    CLASS_STUDENT_SERVICE
        .delete_class_user(&request, class_id, class_user_id)
        .await
}

// 配置路由
pub fn configure_class_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/classes/{class_id}/students")
            .wrap(middlewares::RequireJWT)
            .service(
                web::resource("")
                    .route(
                        web::post()
                            .to(join_class)
                            // 学生加入班级，需要传入班级 ID 和邀请码，User / Teacher 权限
                            .wrap(middlewares::RequireRole::new_any(UserRole::user_roles())),
                    )
                    .route(
                        web::get()
                            .to(list_class_users_with_pagination)
                            // 列出班级学生，Class_Representative 或更高权限
                            .wrap(middlewares::RequireClassRole::new_any(
                                ClassUserRole::class_representative_roles(),
                            )),
                    ),
            )
            .service(
                web::resource("/{class_user_id}")
                    .route(
                        web::get()
                            .to(get_class_user)
                            // 获取班级指定学生详细信息
                            .wrap(middlewares::RequireClassRole::new_any(
                                ClassUserRole::all_roles(),
                            )),
                    )
                    .route(
                        web::put()
                            .to(update_class_user)
                            .wrap(middlewares::RequireRole::new_any(UserRole::teacher_roles())),
                    )
                    .route(
                        web::delete()
                            // 删除班级指定用户，用户自己或者班级教师权限
                            .to(delete_class_user),
                    ),
            ),
    );
}
