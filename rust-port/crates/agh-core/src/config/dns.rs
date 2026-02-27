use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde(default = "default_bind_hosts")]
    pub bind_hosts: Vec<String>,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_upstreams")]
    pub upstream_dns: Vec<String>,
    #[serde(default)]
    pub upstream_dns_file: String,
    #[serde(default = "default_bootstrap")]
    pub bootstrap_dns: Vec<String>,
    #[serde(default)]
    pub fallback_dns: Vec<String>,
    #[serde(default = "default_true")]
    pub all_servers: bool,
    #[serde(default)]
    pub fastest_addr: bool,
    #[serde(default = "default_true")]
    pub fastest_timeout: bool,
    #[serde(default)]
    pub allowed_clients: Vec<String>,
    #[serde(default)]
    pub disallowed_clients: Vec<String>,
    #[serde(default)]
    pub blocked_hosts: Vec<String>,
    #[serde(default)]
    pub trusted_proxies: Vec<String>,
    #[serde(default = "default_cache_size")]
    pub cache_size: u32,
    #[serde(default)]
    pub cache_ttl_min: u32,
    #[serde(default)]
    pub cache_ttl_max: u32,
    #[serde(default)]
    pub cache_optimistic: bool,
    #[serde(default)]
    pub bogus_nxdomain: Vec<String>,
    #[serde(default)]
    pub aaaa_disabled: bool,
    #[serde(default = "default_true")]
    pub enable_dnssec: bool,
    #[serde(default)]
    pub edns_client_subnet: EdnsClientSubnet,
    #[serde(default)]
    pub filtering_enabled: bool,
    #[serde(default)]
    pub filters_update_interval: u32,
    #[serde(default)]
    pub parental_enabled: bool,
    #[serde(default)]
    pub safebrowsing_enabled: bool,
    #[serde(default)]
    pub safebrowsing_cache_size: u32,
    #[serde(default)]
    pub safesearch_cache_size: u32,
    #[serde(default)]
    pub parental_cache_size: u32,
    #[serde(default)]
    pub safe_search: SafeSearchConfig,
    #[serde(default)]
    pub rewrites: Vec<DnsRewrite>,
    #[serde(default)]
    pub blocked_services: BlockedServicesConfig,
    #[serde(default)]
    pub local_ptr_upstreams: Vec<String>,
    #[serde(default = "default_true")]
    pub use_private_ptr_resolvers: bool,
    #[serde(default = "default_true")]
    pub resolve_clients: bool,
    #[serde(default = "default_upstream_mode")]
    pub upstream_mode: String,
    #[serde(default)]
    pub local_domain_name: String,
    #[serde(default)]
    pub private_networks: Vec<String>,
}

fn default_bind_hosts() -> Vec<String> {
    vec!["0.0.0.0".to_string()]
}
fn default_port() -> u16 {
    53
}
fn default_upstreams() -> Vec<String> {
    vec!["https://dns10.quad9.net/dns-query".to_string()]
}
fn default_bootstrap() -> Vec<String> {
    vec![
        "9.9.9.10".to_string(),
        "149.112.112.10".to_string(),
        "2620:fe::10".to_string(),
        "2620:fe::fe:10".to_string(),
    ]
}
fn default_true() -> bool {
    true
}
fn default_cache_size() -> u32 {
    4194304
}
fn default_upstream_mode() -> String {
    "load_balance".to_string()
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            bind_hosts: default_bind_hosts(),
            port: default_port(),
            upstream_dns: default_upstreams(),
            upstream_dns_file: String::new(),
            bootstrap_dns: default_bootstrap(),
            fallback_dns: Vec::new(),
            all_servers: false,
            fastest_addr: false,
            fastest_timeout: true,
            allowed_clients: Vec::new(),
            disallowed_clients: Vec::new(),
            blocked_hosts: Vec::new(),
            trusted_proxies: Vec::new(),
            cache_size: default_cache_size(),
            cache_ttl_min: 0,
            cache_ttl_max: 0,
            cache_optimistic: false,
            bogus_nxdomain: Vec::new(),
            aaaa_disabled: false,
            enable_dnssec: false,
            edns_client_subnet: EdnsClientSubnet::default(),
            filtering_enabled: true,
            filters_update_interval: 24,
            parental_enabled: false,
            safebrowsing_enabled: false,
            safebrowsing_cache_size: 1048576,
            safesearch_cache_size: 1048576,
            parental_cache_size: 1048576,
            safe_search: SafeSearchConfig::default(),
            rewrites: Vec::new(),
            blocked_services: BlockedServicesConfig::default(),
            local_ptr_upstreams: Vec::new(),
            use_private_ptr_resolvers: true,
            resolve_clients: true,
            upstream_mode: default_upstream_mode(),
            local_domain_name: "lan".to_string(),
            private_networks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EdnsClientSubnet {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub use_custom: bool,
    #[serde(default)]
    pub custom_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeSearchConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub bing: bool,
    #[serde(default = "default_true")]
    pub duckduckgo: bool,
    #[serde(default = "default_true")]
    pub google: bool,
    #[serde(default = "default_true")]
    pub pixabay: bool,
    #[serde(default = "default_true")]
    pub yandex: bool,
    #[serde(default = "default_true")]
    pub youtube: bool,
}

impl Default for SafeSearchConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bing: true,
            duckduckgo: true,
            google: true,
            pixabay: true,
            yandex: true,
            youtube: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DnsRewrite {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub answer: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockedServicesConfig {
    #[serde(default)]
    pub schedule: serde_json::Value,
    #[serde(default)]
    pub ids: Vec<String>,
}
