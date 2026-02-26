pub mod aggregation;
pub mod storage;

/// Stats errors.
#[derive(thiserror::Error, Debug)]
pub enum StatsError {
    #[error("Database error: {0}")]
    Db(#[from] redb::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<redb::DatabaseError> for StatsError {
    fn from(e: redb::DatabaseError) -> Self {
        StatsError::Db(e.into())
    }
}

impl From<redb::StorageError> for StatsError {
    fn from(e: redb::StorageError) -> Self {
        StatsError::Db(e.into())
    }
}

impl From<redb::TableError> for StatsError {
    fn from(e: redb::TableError) -> Self {
        StatsError::Db(e.into())
    }
}

impl From<redb::CommitError> for StatsError {
    fn from(e: redb::CommitError) -> Self {
        StatsError::Db(e.into())
    }
}
