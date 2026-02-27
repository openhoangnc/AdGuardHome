use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub server_name: String,
    #[serde(default)]
    pub force_https: bool,
    #[serde(default = "default_port_https")]
    pub port_https: u16,
    #[serde(default = "default_port_dns_over_tls")]
    pub port_dns_over_tls: u16,
    #[serde(default = "default_port_dns_over_quic")]
    pub port_dns_over_quic: u16,
    #[serde(default)]
    pub certificate_chain: String,
    #[serde(default)]
    pub private_key: String,
    #[serde(default)]
    pub certificate_path: String,
    #[serde(default)]
    pub private_key_path: String,
    #[serde(default)]
    pub strict_sni_check: bool,
}

fn default_port_https() -> u16 {
    443
}
fn default_port_dns_over_tls() -> u16 {
    853
}
fn default_port_dns_over_quic() -> u16 {
    784
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_name: String::new(),
            force_https: false,
            port_https: 443,
            port_dns_over_tls: 853,
            port_dns_over_quic: 784,
            certificate_chain: String::new(),
            private_key: String::new(),
            certificate_path: String::new(),
            private_key_path: String::new(),
            strict_sni_check: false,
        }
    }
}
