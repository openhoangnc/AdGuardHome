pub mod arp;
pub mod leases;
pub mod v4;
pub mod v6;

/// DHCP server errors.
#[derive(thiserror::Error, Debug)]
pub enum DhcpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No available addresses in pool")]
    PoolExhausted,
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
