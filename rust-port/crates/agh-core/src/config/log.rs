use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub max_backups: i32,
    #[serde(default = "default_log_max_size")]
    pub max_size: i32,
    #[serde(default)]
    pub max_age: i32,
    #[serde(default)]
    pub compress: bool,
    #[serde(default)]
    pub local_time: bool,
    #[serde(default)]
    pub verbose: bool,
}

fn default_true() -> bool { true }
fn default_log_max_size() -> i32 { 100 }

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file: String::new(),
            max_backups: 0,
            max_size: 100,
            max_age: 0,
            compress: false,
            local_time: false,
            verbose: false,
        }
    }
}
