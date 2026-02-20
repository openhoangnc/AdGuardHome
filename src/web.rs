use axum::{
    routing::get,
    Router,
};
use tracing::info;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use crate::config::HttpConfig;

pub async fn start(config: HttpConfig) -> anyhow::Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello from AdGuardHome Rust Backend!" }));

    // Use default address if none provided
    let addr_str = if config.addresses.is_empty() {
        "0.0.0.0:3000".to_string()
    } else {
        config.addresses[0].clone()
    };

    let addr: SocketAddr = addr_str.parse().unwrap_or_else(|_| {
        info!("Invalid address {}, using default 0.0.0.0:3000", addr_str);
        "0.0.0.0:3000".parse().unwrap()
    });

    info!("Starting web server on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
