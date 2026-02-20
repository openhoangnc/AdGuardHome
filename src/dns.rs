use crate::config::DnsConfig;
use tracing::info;

pub struct Server {
    config: DnsConfig,
}

impl Server {
    pub fn new(config: DnsConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        info!("Starting DNS server with config: {:?}", self.config);
        // In a real implementation, this would start listening on UDP/TCP ports.
        // For now, it just logs and exits (or stays running if I make it async loop).

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            info!("DNS server is running...");
        }
    }
}
