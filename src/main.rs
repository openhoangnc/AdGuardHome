mod config;
mod web;
mod dns;

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "AdGuardHome.yaml")]
    config: String,

    /// Working directory
    #[arg(short, long)]
    work_dir: Option<String>,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if args.verbose { Level::DEBUG } else { Level::INFO })
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    info!("Starting AdGuard Home Rust Backend...");

    // Change working directory if specified
    if let Some(work_dir) = &args.work_dir {
        std::env::set_current_dir(work_dir)
            .with_context(|| format!("Failed to change working directory to {}", work_dir))?;
        info!("Changed working directory to {}", work_dir);
    }

    // Load configuration
    let config = config::load(&args.config)?;
    info!("Configuration loaded successfully");

    // Extract configurations
    let dns_config = config.dns;
    let http_config = config.http;

    // Start services
    let dns_server = dns::Server::new(dns_config);

    info!("Starting services...");

    tokio::select! {
        _ = dns_server.run() => {
            info!("DNS server stopped unexpectedly");
        }
        res = web::start(http_config) => {
            if let Err(e) = res {
                tracing::error!("Web server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down");
        }
    }

    Ok(())
}
