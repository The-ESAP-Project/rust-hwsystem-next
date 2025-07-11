use crate::services::homeworks::homeworks::list_homeworks;
use actix_web::web; // 引入你的 handler

pub fn configure_homeworks_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/homeworks").route("", web::get().to(list_homeworks)), // GET /api/v1/homeworks
    );
}
