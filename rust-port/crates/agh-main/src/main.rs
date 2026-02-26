use std::sync::Arc;

use agh_core::cli::Cli;
use agh_core::config_io::ConfigManager;
use agh_web::auth::SessionStore;
use agh_web::routes::AppState;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    agh_core::cli::init_tracing(cli.verbose, cli.logfile.as_deref());

    tracing::info!("AdGuardHome starting (Rust port) v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration.
    let config = Arc::new(ConfigManager::load(&cli.config).await?);
    if config.is_first_run() {
        tracing::info!("First run — default config will be saved after setup");
    }

    // Build application state.
    let state = AppState {
        config: config.clone(),
        sessions: Arc::new(SessionStore::new()),
    };

    // Determine bind address.
    let bind_addr = {
        let cfg = config.get_async().await;
        if let Some(host) = cli.host {
            let port = cli.port.unwrap_or_else(|| {
                cfg.http.address.rsplit(':').next()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(3000)
            });
            format!("{host}:{port}")
        } else {
            cfg.http.address.clone()
        }
    };

    // Build and start the HTTP server.
    let router = agh_web::create_router(state);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!(address = %bind_addr, "HTTP server listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c().await.expect("ctrl-c handler");
    };

    #[cfg(unix)]
    {
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("SIGTERM handler")
                .recv()
                .await;
        };
        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }

    #[cfg(not(unix))]
    ctrl_c.await;

    tracing::info!("Shutting down...");
}

