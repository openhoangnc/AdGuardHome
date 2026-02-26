pub mod download;
pub mod version;

#[derive(thiserror::Error, Debug)]
pub enum UpdaterError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
