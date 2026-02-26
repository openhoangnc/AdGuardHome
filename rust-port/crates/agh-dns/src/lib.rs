pub mod cache;
pub mod dnscrypt;
pub mod doh;
pub mod dot;
pub mod doq;
pub mod filtering_handler;
pub mod server;
pub mod upstream;

/// Errors for the agh-dns crate.
#[derive(thiserror::Error, Debug)]
pub enum DnsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("DNS encode error: {0}")]
    Encode(String),
    #[error("DNS decode error: {0}")]
    Decode(String),
    #[error("Upstream error: {0}")]
    Upstream(String),
    #[error("Resolver error: {0}")]
    Resolver(String),
}
