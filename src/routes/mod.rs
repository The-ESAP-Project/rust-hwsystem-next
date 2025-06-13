mod auth;
mod health;

use actix_web::web;
use std::env;

use log::warn;

// 获取API前缀
fn get_api_prefix() -> String {
    env::var("API_PREFIX").unwrap_or_else(|_| {
        warn!("API_PREFIX环境变量未设置，使用默认前缀");
        "/api".to_string()
    })
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let prefix = get_api_prefix();
    
    // 使用范围(scope)将前缀应用到所有路由
    cfg.service(
        web::scope(&prefix)
            .configure(health::configure)
            .configure(auth::configure)
    );
}