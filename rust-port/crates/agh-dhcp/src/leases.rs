//! DHCP lease persistence.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// A DHCP lease entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lease {
    pub mac: String,
    pub ip: String,
    pub hostname: String,
    pub expires: DateTime<Utc>,
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
        let content = serde_json::to_string_pretty(&*leases).map_err(std::io::Error::other)?;
        tokio::fs::write(&self.path, content).await
    }

    pub async fn all(&self) -> Vec<Lease> {
        self.leases.read().await.clone()
    }

    pub async fn add_or_update(&self, lease: Lease) -> std::io::Result<()> {
        {
            let mut leases = self.leases.write().await;
            if let Some(existing) = leases.iter_mut().find(|l| l.mac == lease.mac) {
                *existing = lease;
            } else {
                leases.push(lease);
            }
        }
        self.save().await
    }

    pub async fn remove_by_mac(&self, mac: &str) -> std::io::Result<()> {
        self.leases.write().await.retain(|l| l.mac != mac);
        self.save().await
    }

    /// Find a lease by MAC address (or DUID for DHCPv6).
    pub async fn find_by_mac(&self, mac: &str) -> Option<Lease> {
        self.leases
            .read()
            .await
            .iter()
            .find(|l| l.mac == mac)
            .cloned()
    }

    /// Return true if the given IP string is already assigned to any lease.
    pub async fn is_ip_taken(&self, ip: &str) -> bool {
        self.leases.read().await.iter().any(|l| l.ip == ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lease(mac: &str, ip: &str) -> Lease {
        Lease {
            mac: mac.to_owned(),
            ip: ip.to_owned(),
            hostname: "test-host".to_owned(),
            expires: Utc::now() + chrono::Duration::hours(24),
            is_static: false,
        }
    }

    #[tokio::test]
    async fn test_lease_add_and_retrieve() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("leases.json");
        let store = LeaseStore::load(&path).await.expect("load");
        store
            .add_or_update(make_lease("AA:BB:CC:DD:EE:FF", "192.168.1.100"))
            .await
            .expect("add");
        let all = store.all().await;
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].hostname, "test-host");
    }

    #[tokio::test]
    async fn test_find_by_mac() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("leases.json");
        let store = LeaseStore::load(&path).await.expect("load");
        store
            .add_or_update(make_lease("DE:AD:BE:EF:00:01", "10.0.0.1"))
            .await
            .expect("add");
        let found = store.find_by_mac("DE:AD:BE:EF:00:01").await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().ip, "10.0.0.1");
    }

    #[tokio::test]
    async fn test_is_ip_taken() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("leases.json");
        let store = LeaseStore::load(&path).await.expect("load");
        store
            .add_or_update(make_lease("01:02:03:04:05:06", "172.16.0.5"))
            .await
            .expect("add");
        assert!(store.is_ip_taken("172.16.0.5").await);
        assert!(!store.is_ip_taken("172.16.0.6").await);
    }

    #[tokio::test]
    async fn test_remove_by_mac() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("leases.json");
        let store = LeaseStore::load(&path).await.expect("load");
        store
            .add_or_update(make_lease("AA:AA:AA:AA:AA:AA", "192.168.0.1"))
            .await
            .expect("add");
        assert_eq!(store.all().await.len(), 1);
        store
            .remove_by_mac("AA:AA:AA:AA:AA:AA")
            .await
            .expect("remove");
        assert_eq!(store.all().await.len(), 0);
    }
}
