pub mod query;
pub mod storage;

#[derive(thiserror::Error, Debug)]
pub enum QueryLogError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
