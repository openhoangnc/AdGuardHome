use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default)]
    pub session_ttl: String,
}

fn default_address() -> String { "0.0.0.0:3000".to_string() }

impl Default for HttpConfig {
    fn default() -> Self {
        Self { address: default_address(), session_ttl: "720h".to_string() }
    }
}
