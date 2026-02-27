pub mod clients;
pub mod dhcp;
pub mod dns;
pub mod filter;
pub mod http;
pub mod log;
pub mod os_cfg;
pub mod querylog;
pub mod stats;
pub mod tls;
pub mod user;

use serde::{Deserialize, Serialize};

pub use clients::ClientsConfig;
pub use dhcp::DhcpConfig;
pub use dns::DnsConfig;
pub use filter::FilterConfig;
pub use http::HttpConfig;
pub use log::LogConfig;
pub use os_cfg::OsConfig;
pub use querylog::QueryLogConfig;
pub use stats::StatisticsConfig;
pub use tls::TlsConfig;
pub use user::User;

/// Root configuration structure matching AdGuardHome.yaml schema exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdGuardHomeConfig {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,

    #[serde(default)]
    pub http: HttpConfig,

    #[serde(default)]
    pub users: Vec<User>,

    #[serde(default = "default_auth_attempts")]
    pub auth_attempts: u32,

    #[serde(default = "default_block_auth_min")]
    pub block_auth_min: u32,

    #[serde(default)]
    pub dns: DnsConfig,

    #[serde(default)]
    pub tls: TlsConfig,

    #[serde(default)]
    pub filters: Vec<FilterConfig>,

    #[serde(default)]
    pub whitelist_filters: Vec<FilterConfig>,

    #[serde(default)]
    pub user_rules: Vec<String>,

    #[serde(default)]
    pub dhcp: DhcpConfig,

    #[serde(default)]
    pub clients: ClientsConfig,

    #[serde(default)]
    pub log: LogConfig,

    #[serde(default, rename = "os")]
    pub os: OsConfig,

    #[serde(default)]
    pub statistics: StatisticsConfig,

    #[serde(default)]
    pub querylog: QueryLogConfig,
}

fn default_schema_version() -> u32 {
    28
}
fn default_auth_attempts() -> u32 {
    5
}
fn default_block_auth_min() -> u32 {
    15
}

impl Default for AdGuardHomeConfig {
    fn default() -> Self {
        Self {
            schema_version: default_schema_version(),
            http: HttpConfig::default(),
            users: Vec::new(),
            auth_attempts: default_auth_attempts(),
            block_auth_min: default_block_auth_min(),
            dns: DnsConfig::default(),
            tls: TlsConfig::default(),
            filters: Vec::new(),
            whitelist_filters: Vec::new(),
            user_rules: Vec::new(),
            dhcp: DhcpConfig::default(),
            clients: ClientsConfig::default(),
            log: LogConfig::default(),
            os: OsConfig::default(),
            statistics: StatisticsConfig::default(),
            querylog: QueryLogConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_serializes() {
        let cfg = AdGuardHomeConfig::default();
        let yaml = serde_yaml::to_string(&cfg).expect("serialize");
        let roundtrip: AdGuardHomeConfig = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(cfg.schema_version, roundtrip.schema_version);
        assert_eq!(cfg.auth_attempts, roundtrip.auth_attempts);
    }

    #[test]
    fn test_partial_config_deserializes() {
        let yaml = "schema_version: 28\nhttp:\n  address: 0.0.0.0:3000\n";
        let cfg: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("deserialize");
        assert_eq!(cfg.http.address, "0.0.0.0:3000");
    }
}
