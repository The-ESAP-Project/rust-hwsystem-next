use actix_web::{get, web, HttpResponse, Responder};

// 健康检查
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}