//! Append-only query log storage with rotation.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

/// A single query log entry (JSON-lines format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLogEntry {
    #[serde(rename = "T")]
    pub time: DateTime<Utc>,
    #[serde(rename = "QH")]
    pub question_host: String,
    #[serde(rename = "QT")]
    pub question_type: String,
    #[serde(rename = "QC")]
    pub question_class: String,
    #[serde(rename = "CP")]
    pub client: String,
    #[serde(rename = "Result")]
    pub result: serde_json::Value,
    #[serde(rename = "Elapsed")]
    pub elapsed_ms: u64,
    #[serde(rename = "Upstream")]
    pub upstream: String,
}

/// Append-only query log with file rotation.
pub struct QueryLogStorage {
    path: PathBuf,
    #[allow(dead_code)]
    max_size_bytes: u64,
    file: Arc<Mutex<Option<tokio::fs::File>>>,
}

impl QueryLogStorage {
    pub async fn open(path: &Path, max_size_mb: u64) -> Result<Self, super::QueryLogError> {
        let file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        Ok(Self {
            path: path.to_owned(),
            max_size_bytes: max_size_mb * 1024 * 1024,
            file: Arc::new(Mutex::new(Some(file))),
        })
    }

    /// Append a new entry to the log.
    pub async fn append(&self, entry: &QueryLogEntry) -> Result<(), super::QueryLogError> {
        let mut line = serde_json::to_string(entry)?;
        line.push('\n');
        let mut guard = self.file.lock().await;
        if let Some(f) = guard.as_mut() {
            f.write_all(line.as_bytes()).await?;
        }
        Ok(())
    }

    /// Read all entries from the log file (most recent first).
    pub async fn read_all(&self) -> Result<Vec<QueryLogEntry>, super::QueryLogError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let content = tokio::fs::read_to_string(&self.path).await?;
        let mut entries: Vec<QueryLogEntry> = content
            .lines()
            .filter(|l| !l.is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        entries.reverse(); // Most recent first.
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_append_and_read() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("querylog.json");
        let storage = QueryLogStorage::open(&path, 10).await.expect("open");
        let entry = QueryLogEntry {
            time: Utc::now(),
            question_host: "example.com".to_owned(),
            question_type: "A".to_owned(),
            question_class: "IN".to_owned(),
            client: "192.168.1.1".to_owned(),
            result: serde_json::json!({"IsFiltered": false}),
            elapsed_ms: 5,
            upstream: "8.8.8.8".to_owned(),
        };
        storage.append(&entry).await.expect("append");
        let all = storage.read_all().await.expect("read");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].question_host, "example.com");
    }
}
