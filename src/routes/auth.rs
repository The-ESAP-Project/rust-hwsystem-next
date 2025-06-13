use actix_web::{get, post, web, HttpResponse, Responder};
use crate::utils::jwt_utils::JwtMiddleware;

use crate::models::LoginRequest;

// 受JWT保护的路由
#[get("/protected")]
async fn protected_route(jwt: JwtMiddleware) -> impl Responder {
    // jwt.claims 中包含已验证的用户信息
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "这是一个受保护的资源",
        "username": jwt.claims.username,
        "name": jwt.claims.name,
        "role": jwt.claims.role
    }))
}

// 登录示例 - 生成JWT令牌
#[post("/auth/login")]
async fn login(req: web::Json<LoginRequest>,) -> impl Responder {
    use crate::utils::jwt_utils::generate_token;
    use std::time::Duration;
    
    let (username, password) = (req.username.clone(), req.password.clone());

    // 检查用户名和密码
    let username = "AptS_1547";
    let name = "卞雨涵";
    let major_class = "计算机科学与技术";
    let role = "admin";
    
    // 生成一个有效期为24小时的令牌
    match generate_token("access_token", username, name, major_class, role, Duration::from_secs(86400)) {
        Ok(token) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "token": token
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "无法生成令牌"
        })),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(protected_route)
       .service(login);
}