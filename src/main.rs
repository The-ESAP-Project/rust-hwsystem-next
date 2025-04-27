use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::sync::{Arc, Mutex};

// 配置结构体
#[derive(Clone, Debug)]
struct Config {
    server_host: String,
    server_port: u16,
    database_url: String,
    jwt_secret: String,
}

// 应用状态
struct AppState {
    config: Config,
    request_count: Mutex<u64>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    config: ConfigInfo,
    requests: u64,
}

#[derive(Serialize)]
struct ConfigInfo {
    host: String,
    port: u16,
}

#[get("/status")]
async fn status(data: web::Data<Arc<AppState>>) -> impl Responder {
    // 使用不可变引用访问配置
    let config = &data.config;
    
    // 使用可变引用修改请求计数器
    let mut count = data.request_count.lock().unwrap();
    *count += 1;
    
    HttpResponse::Ok().json(StatusResponse {
        status: "running".to_string(),
        config: ConfigInfo {
            host: config.server_host.clone(),
            port: config.server_port,
        },
        requests: *count,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 从环境变量读取配置
    let config = Config {
        server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        server_port: env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    };
    
    let bind_address = format!("{}:{}", config.server_host, config.server_port);
    println!("启动服务器，监听 {}", bind_address);
    
    // 创建应用状态并使用 Arc 包装以便共享
    let app_state = Arc::new(AppState {
        config: config.clone(),
        request_count: Mutex::new(0),
    });
    
    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(status)
    })
    .bind(bind_address)?
    .run()
    .await
}