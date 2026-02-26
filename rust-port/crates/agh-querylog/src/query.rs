//! Query log pagination and filtering.

use crate::storage::{QueryLogEntry, QueryLogStorage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Query filter parameters.
#[derive(Debug, Default, Deserialize)]
pub struct QueryFilter {
    pub search: Option<String>,
    pub client: Option<String>,
    pub question_type: Option<String>,
    pub response_status: Option<String>,
    pub older_than: Option<chrono::DateTime<chrono::Utc>>,
}

/// A page of query log entries.
#[derive(Debug, Serialize)]
pub struct QueryLogPage {
    pub data: Vec<QueryLogEntry>,
    pub oldest: Option<chrono::DateTime<chrono::Utc>>,
}

/// Query log service with pagination.
pub struct QueryLogService {
    storage: Arc<QueryLogStorage>,
}

impl QueryLogService {
    pub fn new(storage: Arc<QueryLogStorage>) -> Self {
        Self { storage }
    }

    /// Get a page of entries matching the filter.
    pub async fn query(
        &self,
        filter: &QueryFilter,
        limit: usize,
    ) -> Result<QueryLogPage, crate::QueryLogError> {
        let mut entries = self.storage.read_all().await?;

        // Apply filters.
        if let Some(search) = &filter.search {
            let search = search.to_lowercase();
            entries.retain(|e| e.question_host.to_lowercase().contains(&search));
        }
        if let Some(client) = &filter.client {
            entries.retain(|e| e.client == *client);
        }
        if let Some(qt) = &filter.question_type {
            entries.retain(|e| e.question_type == *qt);
        }
        if let Some(older_than) = filter.older_than {
            entries.retain(|e| e.time < older_than);
        }

        let oldest = entries.get(limit.saturating_sub(1)).map(|e| e.time);
        entries.truncate(limit);
        Ok(QueryLogPage { data: entries, oldest })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::QueryLogEntry;
    use chrono::Utc;

    #[tokio::test]
    async fn test_pagination() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("ql.json");
        let storage = Arc::new(
            QueryLogStorage::open(&path, 10).await.expect("open")
        );

        for i in 0..10 {
            storage.append(&QueryLogEntry {
                time: Utc::now(),
                question_host: format!("host{i}.example.com"),
                question_type: "A".to_owned(),
                question_class: "IN".to_owned(),
                client: "1.2.3.4".to_owned(),
                result: serde_json::json!({}),
                elapsed_ms: i,
                upstream: "8.8.8.8".to_owned(),
            }).await.expect("append");
        }

        let svc = QueryLogService::new(storage);
        let page = svc.query(&QueryFilter::default(), 5).await.expect("query");
        assert_eq!(page.data.len(), 5);
    }
}
