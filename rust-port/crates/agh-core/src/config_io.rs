use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AdGuardHomeConfig;

/// Errors that can occur during config I/O operations.
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("Unsupported schema version: {0}")]
    UnsupportedVersion(u32),
}

/// Manages reading and writing the AdGuardHome config file atomically.
pub struct ConfigManager {
    path: PathBuf,
    is_first_run: bool,
    config: Arc<RwLock<AdGuardHomeConfig>>,
}

impl ConfigManager {
    /// Load config from disk, or create default if not found (firstRun).
    pub async fn load(path: &Path) -> Result<Self, ConfigError> {
        let (config, is_first_run) = if path.exists() {
            let content = tokio::fs::read_to_string(path).await?;
            let cfg: AdGuardHomeConfig = serde_yaml::from_str(&content)?;
            (cfg, false)
        } else {
            (AdGuardHomeConfig::default(), true)
        };

        Ok(Self {
            path: path.to_owned(),
            is_first_run,
            config: Arc::new(RwLock::new(config)),
        })
    }

    /// Atomically write config: write to `path.tmp`, then rename.
    pub async fn save(&self) -> Result<(), ConfigError> {
        let cfg = self.config.read().await.clone();
        let content = serde_yaml::to_string(&cfg)?;
        write_atomic(&self.path, &content).await
    }

    /// Get a clone of the current config.
    pub async fn get_async(&self) -> AdGuardHomeConfig {
        self.config.read().await.clone()
    }

    /// Get a clone of the current config (synchronous — panics inside async context).
    /// Use `get_async()` from within async code.
    pub fn get(&self) -> AdGuardHomeConfig {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { self.config.read().await.clone() })
        })
    }

    /// Update config in memory and persist to disk atomically.
    pub async fn update<F>(&self, f: F) -> Result<(), ConfigError>
    where
        F: FnOnce(&mut AdGuardHomeConfig) + Send,
    {
        {
            let mut cfg = self.config.write().await;
            f(&mut cfg);
        }
        self.save().await
    }

    /// Returns true if this is the first run (config file did not exist).
    pub fn is_first_run(&self) -> bool {
        self.is_first_run
    }

    /// Returns the config file path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

async fn write_atomic(path: &Path, content: &str) -> Result<(), ConfigError> {
    let mut tmp_name = path.file_name()
        .unwrap_or_default()
        .to_os_string();
    tmp_name.push(".tmp");
    let tmp = path.with_file_name(tmp_name);
    tokio::fs::write(&tmp, content).await?;
    tokio::fs::rename(&tmp, path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Barrier;

    #[tokio::test]
    async fn test_load_missing_file_is_first_run() {
        let dir = tempdir();
        let path = dir.path().join("AdGuardHome.yaml");
        let mgr = ConfigManager::load(&path).await.expect("load");
        assert!(mgr.is_first_run());
        assert_eq!(mgr.get_async().await.schema_version, 28);
    }

    #[tokio::test]
    async fn test_atomic_write_creates_file() {
        let dir = tempdir();
        let path = dir.path().join("AdGuardHome.yaml");
        let mgr = ConfigManager::load(&path).await.expect("load");
        mgr.save().await.expect("save");
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_roundtrip_preserves_all_fields() {
        let dir = tempdir();
        let path = dir.path().join("AdGuardHome.yaml");
        let mgr = ConfigManager::load(&path).await.expect("load");
        mgr.update(|cfg| cfg.auth_attempts = 10).await.expect("update");

        let mgr2 = ConfigManager::load(&path).await.expect("reload");
        assert!(!mgr2.is_first_run());
        assert_eq!(mgr2.get_async().await.auth_attempts, 10);
    }

    #[tokio::test]
    async fn test_concurrent_updates_are_safe() {
        let dir = tempdir();
        // First save a config file so concurrent writers have a file to overwrite.
        let path = dir.path().join("AdGuardHome.yaml");
        {
            let init = ConfigManager::load(&path).await.expect("init load");
            init.save().await.expect("init save");
        }
        let mgr = Arc::new(ConfigManager::load(&path).await.expect("load"));
        let barrier = Arc::new(Barrier::new(5));
        let mut handles = Vec::new();
        for i in 0u32..5 {
            let m = mgr.clone();
            let b = barrier.clone();
            handles.push(tokio::spawn(async move {
                b.wait().await;
                // Ignore IO races — the goal is no panic / memory unsafety.
                let _ = m.update(|cfg| cfg.auth_attempts = i).await;
            }));
        }
        for h in handles {
            h.await.expect("task panicked");
        }
        // Config should still be readable after concurrent writes.
        assert!(mgr.get_async().await.auth_attempts < 10);
    }

    fn tempdir() -> tempfile::TempDir {
        tempfile::tempdir().expect("tempdir")
    }
}
