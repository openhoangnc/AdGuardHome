use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};

use ipnet::IpNet;
use serde::{Deserialize, Serialize};

/// Error type for client registry operations.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Client with name '{0}' already exists")]
    DuplicateName(String),
    #[error("Client ID '{0}' is already used by client '{1}'")]
    DuplicateId(String, String),
    #[error("Client '{0}' not found")]
    NotFound(String),
}

/// Safe-search configuration per client.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SafeSearchConfig {
    pub enabled: bool,
    pub bing: bool,
    pub duckduckgo: bool,
    pub google: bool,
    pub pixabay: bool,
    pub yandex: bool,
    pub youtube: bool,
}

/// A persistent client configured by the admin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentClient {
    pub name: String,
    /// IP addresses, MAC addresses, or CIDRs that identify this client.
    pub ids: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub blocked_services: Vec<String>,
    #[serde(default)]
    pub upstreams: Vec<String>,
    #[serde(default = "default_true")]
    pub filtering_enabled: bool,
    #[serde(default)]
    pub parental_enabled: bool,
    #[serde(default)]
    pub safebrowsing_enabled: bool,
    #[serde(default)]
    pub safesearch: SafeSearchConfig,
    #[serde(default = "default_true")]
    pub use_global_settings: bool,
    #[serde(default = "default_true")]
    pub use_global_blocked_services: bool,
    #[serde(default)]
    pub ignore_querylog: bool,
    #[serde(default)]
    pub ignore_statistics: bool,
}

fn default_true() -> bool { true }

/// Source of a runtime-discovered client.
#[derive(Debug, Clone)]
pub enum ClientSource {
    Rdns,
    Dhcp,
    Arp,
    Hosts,
}

/// A runtime client discovered automatically (not configured by the admin).
#[derive(Debug, Clone)]
pub struct RuntimeClient {
    pub ip: IpAddr,
    pub name: Option<String>,
    pub source: ClientSource,
}

/// In-memory registry of persistent and runtime clients.
pub struct ClientRegistry {
    persistent: Arc<RwLock<Vec<PersistentClient>>>,
    runtime: Arc<RwLock<HashMap<IpAddr, RuntimeClient>>>,
}

impl ClientRegistry {
    /// Create a registry pre-populated with the given persistent clients.
    pub fn new(clients: Vec<PersistentClient>) -> Self {
        Self {
            persistent: Arc::new(RwLock::new(clients)),
            runtime: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Find a persistent client whose `ids` match the given IP.
    pub fn find_persistent(&self, ip: &IpAddr) -> Option<PersistentClient> {
        let lock = self.persistent.read().expect("lock poisoned");
        lock.iter()
            .find(|c| client_matches(c, ip))
            .cloned()
    }

    /// Find a runtime client by exact IP.
    pub fn find_runtime(&self, ip: &IpAddr) -> Option<RuntimeClient> {
        self.runtime.read().expect("lock poisoned").get(ip).cloned()
    }

    /// Register or update a runtime client.
    pub fn add_runtime(&self, client: RuntimeClient) {
        self.runtime.write().expect("lock poisoned").insert(client.ip, client);
    }

    /// Return all persistent clients.
    pub fn list_persistent(&self) -> Vec<PersistentClient> {
        self.persistent.read().expect("lock poisoned").clone()
    }

    /// Add a new persistent client, rejecting duplicates.
    pub fn add_persistent(&self, client: PersistentClient) -> Result<(), ClientError> {
        let mut lock = self.persistent.write().expect("lock poisoned");
        // Check name uniqueness.
        if lock.iter().any(|c| c.name == client.name) {
            return Err(ClientError::DuplicateName(client.name.clone()));
        }
        // Check ID uniqueness.
        for id in &client.ids {
            if let Some(existing) = lock.iter().find(|c| c.ids.contains(id)) {
                return Err(ClientError::DuplicateId(id.clone(), existing.name.clone()));
            }
        }
        lock.push(client);
        Ok(())
    }

    /// Remove a persistent client by name.
    pub fn remove_persistent(&self, name: &str) -> Result<(), ClientError> {
        let mut lock = self.persistent.write().expect("lock poisoned");
        let pos = lock.iter().position(|c| c.name == name)
            .ok_or_else(|| ClientError::NotFound(name.to_owned()))?;
        lock.remove(pos);
        Ok(())
    }

    /// Update a persistent client by name.
    pub fn update_persistent(&self, name: &str, client: PersistentClient) -> Result<(), ClientError> {
        let mut lock = self.persistent.write().expect("lock poisoned");
        let pos = lock.iter().position(|c| c.name == name)
            .ok_or_else(|| ClientError::NotFound(name.to_owned()))?;
        lock[pos] = client;
        Ok(())
    }
}

/// Returns true if the client's `ids` match the given IP.
fn client_matches(client: &PersistentClient, ip: &IpAddr) -> bool {
    let ip_str = ip.to_string();
    for id in &client.ids {
        // Exact IP match.
        if id == &ip_str {
            return true;
        }
        // CIDR match.
        if let Ok(net) = id.parse::<IpNet>() {
            if net.contains(ip) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_client(name: &str, ids: &[&str]) -> PersistentClient {
        PersistentClient {
            name: name.to_owned(),
            ids: ids.iter().map(|s| s.to_string()).collect(),
            tags: Vec::new(),
            blocked_services: Vec::new(),
            upstreams: Vec::new(),
            filtering_enabled: true,
            parental_enabled: false,
            safebrowsing_enabled: false,
            safesearch: SafeSearchConfig::default(),
            use_global_settings: true,
            use_global_blocked_services: true,
            ignore_querylog: false,
            ignore_statistics: false,
        }
    }

    #[test]
    fn test_find_by_ip_exact() {
        let reg = ClientRegistry::new(vec![make_client("alice", &["192.168.1.10"])]);
        let ip: IpAddr = "192.168.1.10".parse().unwrap();
        assert!(reg.find_persistent(&ip).is_some());
        let other: IpAddr = "192.168.1.11".parse().unwrap();
        assert!(reg.find_persistent(&other).is_none());
    }

    #[test]
    fn test_find_by_cidr() {
        let reg = ClientRegistry::new(vec![make_client("subnet", &["10.0.0.0/24"])]);
        let ip: IpAddr = "10.0.0.42".parse().unwrap();
        assert!(reg.find_persistent(&ip).is_some());
        let outside: IpAddr = "10.0.1.1".parse().unwrap();
        assert!(reg.find_persistent(&outside).is_none());
    }

    #[test]
    fn test_add_duplicate_name_rejected() {
        let reg = ClientRegistry::new(vec![]);
        reg.add_persistent(make_client("alice", &["1.2.3.4"])).unwrap();
        let err = reg.add_persistent(make_client("alice", &["5.6.7.8"])).unwrap_err();
        assert!(matches!(err, ClientError::DuplicateName(_)));
    }

    #[test]
    fn test_remove_persistent() {
        let reg = ClientRegistry::new(vec![make_client("alice", &["1.2.3.4"])]);
        reg.remove_persistent("alice").unwrap();
        assert!(reg.list_persistent().is_empty());
    }
}
