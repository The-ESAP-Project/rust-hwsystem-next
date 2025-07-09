/*!
 * 基于角色的访问控制中间件
 *
 * 此中间件必须在 RequireJWT 中间件之后使用，用于验证用户是否具有特定角色权限。
 *
 * ## 使用方法
 *
 * ```rust
 * use actix_web::{web, App, HttpServer};
 * use crate::middlewares::require_jwt::RequireJWT;
 * use crate::middlewares::require_role::RequireRole;
 *
 * HttpServer::new(|| {
 *     App::new()
 *         .service(
 *             web::scope("/api")
 *                 .wrap(RequireJWT)  // 先验证JWT
 *                 .service(
 *                     web::scope("/admin")
 *                         .wrap(RequireRole::new("admin"))  // 再验证角色
 *                         .route("/users", web::get().to(admin_users_handler))
 *                 )
 *         )
 * })
 * ```
 *
 * 或者验证多个角色：
 *
 * ```rust
 * .wrap(RequireRole::new_any(&["admin", "moderator"]))  // 任一角色即可
 * ```
 */

use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    http::header::CONTENT_TYPE,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;
use tracing::info;

#[derive(Clone)]
pub struct RequireRole {
    required_roles: Vec<String>,
    require_all: bool, // true表示需要所有角色，false表示任一角色即可
}

impl RequireRole {
    /// 创建需要特定角色的中间件
    pub fn new(role: &str) -> Self {
        Self {
            required_roles: vec![role.to_string()],
            require_all: true,
        }
    }

    /// 创建需要任一角色的中间件
    pub fn new_any(roles: &[&str]) -> Self {
        Self {
            required_roles: roles.iter().map(|r| r.to_string()).collect(),
            require_all: false,
        }
    }

    /// 创建需要所有角色的中间件
    pub fn new_all(roles: &[&str]) -> Self {
        Self {
            required_roles: roles.iter().map(|r| r.to_string()).collect(),
            require_all: true,
        }
    }
}

// 辅助函数：创建错误响应
fn create_error_response(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status)
        .insert_header((CONTENT_TYPE, "application/json; charset=utf-8"))
        .json(serde_json::json!({
            "code": status.as_u16(),
            "data": { "error": message }
        }))
}

impl<S, B> Transform<S, ServiceRequest> for RequireRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireRoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireRoleMiddleware {
            service: Rc::new(service),
            required_roles: self.required_roles.clone(),
            require_all: self.require_all,
        }))
    }
}

pub struct RequireRoleMiddleware<S> {
    service: Rc<S>,
    required_roles: Vec<String>,
    require_all: bool,
}

impl<S, B> Service<ServiceRequest> for RequireRoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let required_roles = self.required_roles.clone();
        let require_all = self.require_all;

        Box::pin(async move {
            // 从请求扩展中获取用户Claims
            let user_claims = req.extensions().get::<crate::utils::jwt::Claims>().cloned();

            match user_claims {
                Some(claims) => {
                    let user_sub = claims.sub.clone();
                    let user_role = &claims.role;
                    let has_permission = if require_all {
                        // 需要所有角色（通常用于单一角色验证）
                        required_roles.iter().all(|role| user_role == role)
                    } else {
                        // 需要任一角色
                        required_roles.iter().any(|role| user_role == role)
                    };

                    if has_permission {
                        let res = srv.call(req).await?.map_into_left_body();
                        Ok(res)
                    } else {
                        info!(
                            "Access denied for user {} (role: {}). Required roles: {:?}",
                            user_sub, user_role, required_roles
                        );
                        Ok(req.into_response(
                            create_error_response(
                                StatusCode::FORBIDDEN,
                                &format!("Access denied. Required role(s): {required_roles:?}"),
                            )
                            .map_into_right_body(),
                        ))
                    }
                }
                None => {
                    info!(
                        "Role check failed: No user claims found in request. Make sure RequireJWT middleware is applied first."
                    );
                    Ok(req.into_response(
                        create_error_response(StatusCode::UNAUTHORIZED, "Authentication required")
                            .map_into_right_body(),
                    ))
                }
            }
        })
    }
}
