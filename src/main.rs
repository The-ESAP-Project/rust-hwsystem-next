use actix_web::{App, HttpServer, middleware::DefaultHeaders, web};
use dotenv::dotenv;
use std::env;
use tracing::{debug, warn};

mod api_models;
mod errors;
mod models;
mod routes;
mod services;
mod storages;
mod system;

use crate::models::{AppConfig, AppStartTime};
use crate::system::lifetime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 记录程序启动时间
    let app_start_time = AppStartTime {
        start_datetime: chrono::Utc::now(),
    };

    // 启动前预处理 //

    debug!("Starting pre-startup processing...");
    // 初始化日志
    let stdout_log = std::io::stdout();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(stdout_log);
    let filter = tracing_subscriber::EnvFilter::new(
        env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    tracing_subscriber::fmt()
        .with_writer(non_blocking_writer)
        .with_env_filter(filter)
        .with_level(true)
        .with_ansi(true)
        .init();

    let startup = lifetime::startup::prepare_server_startup().await;

    let storage = startup.storage.clone();

    // 输出预处理时间
    debug!(
        "Pre-startup processing completed in {} ms",
        chrono::Utc::now()
            .signed_duration_since(app_start_time.start_datetime)
            .num_milliseconds()
    );

    // 预处理完成 //

    // Load env configurations
    let config = AppConfig {
        server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        server_port: env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap(),
        #[cfg(unix)]
        unix_socket_path: env::var("UNIX_SOCKET").ok(),
    };

    let cpu_count = env::var("CPU_COUNT")
        .unwrap_or_else(|_| num_cpus::get().to_string())
        .parse::<usize>()
        .unwrap_or_else(|_| num_cpus::get())
        .min(32);

    warn!("Using {} CPU cores for the server", cpu_count);

    // Start the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storage.clone()))
            .app_data(web::Data::new(app_start_time.clone()))
            .app_data(web::PayloadConfig::new(1024 * 1024)) // 设置最大请求体大小为1MB
            .wrap(
                DefaultHeaders::new()
                    .add(("Connection", "keep-alive"))
                    .add(("Keep-Alive", "timeout=30, max=1000"))
                    .add(("Cache-Control", "no-cache, no-store, must-revalidate")),
            )
            .configure(routes::configure_user_routes)
    })
    .keep_alive(std::time::Duration::from_secs(30)) // 启用长连接
    .client_request_timeout(std::time::Duration::from_millis(5000)) // 客户端超时
    .client_disconnect_timeout(std::time::Duration::from_millis(1000)) // 断连超时
    .workers(cpu_count);

    let server = {
        #[cfg(unix)]
        {
            if let Some(ref socket_path) = config.unix_socket_path {
                warn!("Starting server on Unix socket: {}", socket_path);
                if std::path::Path::new(socket_path).exists() {
                    std::fs::remove_file(socket_path)?;
                }
                Some(server.bind_uds(socket_path)?)
            } else {
                let bind_address = format!("{}:{}", config.server_host, config.server_port);
                warn!("Starting server at http://{}", bind_address);
                Some(server.bind(bind_address)?)
            }
        }

        #[cfg(not(unix))]
        {
            let bind_address = format!("{}:{}", config.server_host, config.server_port);
            warn!("Starting server at http://{}", bind_address);
            Some(server.bind(bind_address)?)
        }
    }
    .expect("Server binding failed")
    .run();

    tokio::select! {
        res = server => {
            res?;
        }
        _ = lifetime::shutdown::listen_for_shutdown() => {
            warn!("Graceful shutdown: all tasks completed");
        }
    }

    Ok(())
}
// DONE
