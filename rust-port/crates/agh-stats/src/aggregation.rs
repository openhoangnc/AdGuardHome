//! Stats aggregation — compute totals across configurable time windows.

use crate::storage::StatsStorage;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Aggregated stats response matching Go's GET /control/stats format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub time_units: String,
    pub num_dns_queries: u64,
    pub num_blocked_filtering: u64,
    pub num_replaced_safebrowsing: u64,
    pub num_replaced_parental: u64,
    pub num_replaced_safesearch: u64,
    pub avg_processing_time: f64,
    pub dns_queries: Vec<u64>,
    pub blocked_filtering: Vec<u64>,
    pub replaced_safebrowsing: Vec<u64>,
    pub replaced_parental: Vec<u64>,
    pub replaced_safesearch: Vec<u64>,
    pub top_queried_domains: Vec<serde_json::Value>,
    pub top_blocked_domains: Vec<serde_json::Value>,
    pub top_clients: Vec<serde_json::Value>,
}

/// Stats aggregation service.
pub struct StatsService {
    storage: Arc<StatsStorage>,
}

impl StatsService {
    pub fn new(storage: Arc<StatsStorage>) -> Self {
        Self { storage }
    }

    pub async fn get_stats(&self) -> StatsResponse {
        let buckets = self.storage.get_all().await;
        let total = self.storage.total_queries().await;
        let blocked = self.storage.total_blocked().await;

        StatsResponse {
            time_units: "hours".to_owned(),
            num_dns_queries: total,
            num_blocked_filtering: blocked,
            num_replaced_safebrowsing: 0,
            num_replaced_parental: 0,
            num_replaced_safesearch: 0,
            avg_processing_time: 0.0,
            dns_queries: buckets.iter().map(|b| b.total_queries).collect(),
            blocked_filtering: buckets.iter().map(|b| b.blocked_queries).collect(),
            replaced_safebrowsing: buckets.iter().map(|b| b.replaced_safebrowsing).collect(),
            replaced_parental: buckets.iter().map(|b| b.replaced_parental).collect(),
            replaced_safesearch: buckets.iter().map(|b| b.replaced_safesearch).collect(),
            top_queried_domains: vec![],
            top_blocked_domains: vec![],
            top_clients: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_stats_empty() {
        let storage = Arc::new(StatsStorage::new(24));
        let svc = StatsService::new(storage);
        let stats = svc.get_stats().await;
        assert_eq!(stats.num_dns_queries, 0);
    }
}
