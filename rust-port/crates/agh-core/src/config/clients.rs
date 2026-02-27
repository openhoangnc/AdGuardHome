use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientsConfig {
    #[serde(default)]
    pub runtime_sources: ClientSourcesConfig,
    #[serde(default)]
    pub persistent: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientSourcesConfig {
    #[serde(default = "default_true")]
    pub whois: bool,
    #[serde(default = "default_true")]
    pub arp: bool,
    #[serde(default = "default_true")]
    pub rdns: bool,
    #[serde(default = "default_true")]
    pub dhcp: bool,
    #[serde(default = "default_true")]
    pub hosts: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ClientSourcesConfig {
    fn default() -> Self {
        Self {
            whois: true,
            arp: true,
            rdns: true,
            dhcp: true,
            hosts: true,
        }
    }
}
