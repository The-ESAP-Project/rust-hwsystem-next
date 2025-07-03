use tokio::signal;
use tracing::warn;

pub async fn listen_for_shutdown() {
    // 等待 Ctrl+C 信号
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    warn!("Shutdown signal received, flushing click data...");
}
