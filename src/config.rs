use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use tracing::info;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub dns: DnsConfig,
    #[serde(default)]
    pub http: HttpConfig,
    #[serde(default)]
    pub log: LogConfig,
    #[serde(default)]
    pub schema_version: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct DnsConfig {
    #[serde(default)]
    pub upstream_mode: String,
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub bootstrap_dns: Vec<String>,
    #[serde(default)]
    pub upstream_dns: Vec<String>,
    #[serde(default)]
    pub upstream_timeout: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct HttpConfig {
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub force_https: bool,
    #[serde(default)]
    pub session_ttl: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct LogConfig {
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub file: String,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    let path = path.as_ref();
    info!("Loading configuration from {}", path.display());

    if !path.exists() {
        info!("Configuration file not found, using defaults");
        return Ok(Config {
            dns: DnsConfig::default(),
            http: HttpConfig::default(),
            log: LogConfig::default(),
            schema_version: 0,
        });
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config = serde_yaml::from_str(&content)
        .with_context(|| "Failed to parse configuration file")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.schema_version, 0);
    }

    #[test]
    fn test_load_nonexistent() {
        let config = load("nonexistent.yaml").unwrap();
        assert_eq!(config.schema_version, 0);
    }
}
