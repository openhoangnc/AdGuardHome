use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::matcher::FilteringEngine;
use crate::parser::parse_filter;

/// Statistics returned after a filter update.
#[derive(Debug, Clone)]
pub struct UpdateStats {
    pub filter_id: u64,
    pub rules_count: usize,
    pub updated_at: DateTime<Utc>,
}

/// Errors that can occur during filter list updates.
#[derive(thiserror::Error, Debug)]
pub enum UpdaterError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Downloads and hot-reloads filter lists.
pub struct FilterUpdater {
    engine: Arc<RwLock<Arc<FilteringEngine>>>,
    http: reqwest::Client,
    cache_dir: PathBuf,
}

impl FilterUpdater {
    /// Create a new updater.
    pub fn new(engine: Arc<RwLock<Arc<FilteringEngine>>>, cache_dir: PathBuf) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("HTTP client");
        Self {
            engine,
            http,
            cache_dir,
        }
    }

    /// Download and reload a single filter list by URL.
    pub async fn update_filter(
        &self,
        filter_id: u64,
        url: &str,
    ) -> Result<UpdateStats, UpdaterError> {
        let content = if url.starts_with("file://") {
            let path = url.trim_start_matches("file://");
            tokio::fs::read_to_string(path).await?
        } else {
            self.http.get(url).send().await?.text().await?
        };

        let rules = parse_filter(&content);
        let rules_count = rules.len();
        let new_engine = Arc::new(FilteringEngine::build(rules));

        // Atomically replace the engine.
        *self.engine.write().await = new_engine;

        // Persist cache.
        let cache_path = self.cache_dir.join(format!("{filter_id}.txt"));
        tokio::fs::write(&cache_path, &content).await?;

        Ok(UpdateStats {
            filter_id,
            rules_count,
            updated_at: Utc::now(),
        })
    }

    /// Update all provided filters.
    pub async fn update_all(
        &self,
        filters: &[(u64, String)],
    ) -> Vec<Result<UpdateStats, UpdaterError>> {
        let mut results = Vec::new();
        for (id, url) in filters {
            results.push(self.update_filter(*id, url).await);
        }
        results
    }

    /// Start a background scheduler that updates all filters on the given interval.
    pub fn start_scheduler(
        self: Arc<Self>,
        filters: Vec<(u64, String)>,
        interval: Duration,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            ticker.tick().await; // skip first immediate tick
            loop {
                ticker.tick().await;
                let results = self.update_all(&filters).await;
                for r in results {
                    match r {
                        Ok(s) => tracing::info!(
                            filter_id = s.filter_id,
                            rules = s.rules_count,
                            "filter updated"
                        ),
                        Err(e) => tracing::warn!(error = %e, "filter update failed"),
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn empty_engine() -> Arc<RwLock<Arc<FilteringEngine>>> {
        Arc::new(RwLock::new(Arc::new(FilteringEngine::build(vec![]))))
    }

    #[tokio::test]
    async fn test_update_from_file_url() {
        let dir = tempfile::tempdir().expect("tempdir");
        let filter_file = dir.path().join("filter.txt");
        tokio::fs::write(&filter_file, "||ads.example.com^\n")
            .await
            .expect("write");
        let url = format!("file://{}", filter_file.display());

        let engine_ref = empty_engine();
        let updater = FilterUpdater::new(engine_ref.clone(), dir.path().to_owned());
        let stats = updater.update_filter(1, &url).await.expect("update");
        assert_eq!(stats.rules_count, 1);
        assert_eq!(stats.filter_id, 1);

        let engine = engine_ref.read().await;
        assert_eq!(engine.rule_count(), 1);
    }

    #[tokio::test]
    async fn test_engine_reloaded_after_parse() {
        let dir = tempfile::tempdir().expect("tempdir");
        let filter_file = dir.path().join("f2.txt");
        tokio::fs::write(&filter_file, "||a.com^\n||b.com^\n||c.com^\n")
            .await
            .expect("write");
        let url = format!("file://{}", filter_file.display());

        let engine_ref = empty_engine();
        let updater = FilterUpdater::new(engine_ref.clone(), dir.path().to_owned());
        let stats = updater.update_filter(2, &url).await.expect("update");
        assert_eq!(stats.rules_count, 3);
        assert_eq!(engine_ref.read().await.rule_count(), 3);
    }
}
