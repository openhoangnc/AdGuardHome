use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_interval")]
    pub interval: String,
    #[serde(default)]
    pub ignored: Vec<String>,
}

fn default_true() -> bool {
    true
}
fn default_interval() -> String {
    "24h".to_string()
}

impl Default for StatisticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: "24h".to_string(),
            ignored: Vec::new(),
        }
    }
}
