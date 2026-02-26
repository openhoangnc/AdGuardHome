//! DHCP lease persistence.


use std::net::IpAddr;
use std::path::{Path, PathBuf};


use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

/// A DHCP lease entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lease {
    pub mac: String,
    pub ip: IpAddr,
    pub hostname: String,
    pub expires: u64,  // Unix timestamp
    pub is_static: bool,
}

/// Persistent lease store backed by a JSON file.
pub struct LeaseStore {
    path: PathBuf,
    leases: Arc<RwLock<Vec<Lease>>>,
}

impl LeaseStore {
    pub async fn load(path: &Path) -> std::io::Result<Self> {
        let leases = if path.exists() {
            let content = tokio::fs::read_to_string(path).await?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        Ok(Self {
            path: path.to_owned(),
            leases: Arc::new(RwLock::new(leases)),
        })
    }

    pub async fn save(&self) -> std::io::Result<()> {
        let leases = self.leases.read().await;
        let content = serde_json::to_string_pretty(&*leases)
            .map_err(std::io::Error::other)?;
        tokio::fs::write(&self.path, content).await
    }

    pub async fn all(&self) -> Vec<Lease> {
        self.leases.read().await.clone()
    }

    pub async fn add_or_update(&self, lease: Lease) {
        let mut leases = self.leases.write().await;
        if let Some(existing) = leases.iter_mut().find(|l| l.mac == lease.mac) {
            *existing = lease;
        } else {
            leases.push(lease);
        }
    }

    pub async fn remove_by_mac(&self, mac: &str) {
        self.leases.write().await.retain(|l| l.mac != mac);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lease_add_and_retrieve() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("leases.json");
        let store = LeaseStore::load(&path).await.expect("load");
        store.add_or_update(Lease {
            mac: "AA:BB:CC:DD:EE:FF".to_owned(),
            ip: "192.168.1.100".parse().unwrap(),
            hostname: "test-host".to_owned(),
            expires: 9999999999,
            is_static: false,
        }).await;
        let all = store.all().await;
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].hostname, "test-host");
    }
}
