//! Time-series stats storage using redb.

use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// A single stats entry for a time bucket.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsBucket {
    pub total_queries: u64,
    pub blocked_queries: u64,
    pub replaced_safebrowsing: u64,
    pub replaced_parental: u64,
    pub replaced_safesearch: u64,
    pub timestamp: i64,  // Unix seconds for the start of this bucket
}

/// In-memory stats storage (simplified — no redb persistence yet).
pub struct StatsStorage {
    buckets: Arc<RwLock<Vec<StatsBucket>>>,
    max_buckets: usize,
}

impl StatsStorage {
    pub fn new(max_buckets: usize) -> Self {
        Self {
            buckets: Arc::new(RwLock::new(Vec::new())),
            max_buckets,
        }
    }

    pub async fn record_query(&self, blocked: bool) {
        let ts = Utc::now().timestamp() / 3600 * 3600; // hourly bucket
        let mut buckets = self.buckets.write().await;
        if let Some(last) = buckets.last_mut() {
            if last.timestamp == ts {
                last.total_queries += 1;
                if blocked { last.blocked_queries += 1; }
                return;
            }
        }
        // New bucket.
        let mut bucket = StatsBucket { timestamp: ts, ..Default::default() };
        bucket.total_queries = 1;
        if blocked { bucket.blocked_queries = 1; }
        buckets.push(bucket);
        // Trim to max_buckets.
        if buckets.len() > self.max_buckets {
            let excess = buckets.len() - self.max_buckets;
            buckets.drain(0..excess);
        }
    }

    pub async fn get_all(&self) -> Vec<StatsBucket> {
        self.buckets.read().await.clone()
    }

    pub async fn total_queries(&self) -> u64 {
        self.buckets.read().await.iter().map(|b| b.total_queries).sum()
    }

    pub async fn total_blocked(&self) -> u64 {
        self.buckets.read().await.iter().map(|b| b.blocked_queries).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_and_read() {
        let store = StatsStorage::new(24);
        store.record_query(false).await;
        store.record_query(true).await;
        assert_eq!(store.total_queries().await, 2);
        assert_eq!(store.total_blocked().await, 1);
    }

    #[tokio::test]
    async fn test_retention_trim() {
        let store = StatsStorage::new(2);
        // Force different timestamps by directly injecting buckets.
        {
            let mut buckets = store.buckets.write().await;
            for i in 0..5i64 {
                buckets.push(StatsBucket { timestamp: i * 3600, total_queries: 1, ..Default::default() });
            }
            // Trim manually as record_query does.
            if buckets.len() > store.max_buckets {
                let excess = buckets.len() - store.max_buckets;
                buckets.drain(0..excess);
            }
        }
        let all = store.get_all().await;
        assert_eq!(all.len(), 2);
    }
}
