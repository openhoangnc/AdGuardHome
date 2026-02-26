use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLogConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_size_mb")]
    pub size_mb: u64,
    #[serde(default)]
    pub ignored: Vec<String>,
    #[serde(default)]
    pub anonymize_client_ip: bool,
    #[serde(default = "default_interval")]
    pub interval: String,
}

fn default_true() -> bool { true }
fn default_size_mb() -> u64 { 100 }
fn default_interval() -> String { "90d".to_string() }

impl Default for QueryLogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 100,
            ignored: Vec::new(),
            anonymize_client_ip: false,
            interval: "90d".to_string(),
        }
    }
}
