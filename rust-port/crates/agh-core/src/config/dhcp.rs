use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DhcpConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub interface_name: String,
    #[serde(default)]
    pub local_domain_name: String,
    #[serde(default)]
    pub dhcpv4: DhcpV4Config,
    #[serde(default)]
    pub dhcpv6: DhcpV6Config,
}

impl Default for DhcpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interface_name: String::new(),
            local_domain_name: "lan".to_string(),
            dhcpv4: DhcpV4Config::default(),
            dhcpv6: DhcpV6Config::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DhcpV4Config {
    #[serde(default)]
    pub gateway_ip: String,
    #[serde(default)]
    pub subnet_mask: String,
    #[serde(default)]
    pub range_start: String,
    #[serde(default)]
    pub range_end: String,
    #[serde(default = "default_lease_duration")]
    pub lease_duration: u64,
    #[serde(default)]
    pub icmp_timeout_msec: u32,
    #[serde(default)]
    pub options: Vec<String>,
}

fn default_lease_duration() -> u64 { 86400 }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DhcpV6Config {
    #[serde(default)]
    pub range_start: String,
    #[serde(default = "default_lease_duration")]
    pub lease_duration: u64,
    #[serde(default = "default_ra_allow_slaac")]
    pub ra_allow_slaac: bool,
    #[serde(default)]
    pub ra_slaac_only: bool,
}

fn default_ra_allow_slaac() -> bool { false }
