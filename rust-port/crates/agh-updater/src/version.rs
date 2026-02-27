//! Version checking against AdGuard's update server.

use serde::{Deserialize, Serialize};

const VERSION_URL_TEMPLATE: &str = "https://static.adguard.com/adguardhome/{channel}/version.json";

/// Version information from the update server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "announcement")]
    pub announcement: String,
    #[serde(rename = "announcement_url")]
    pub announcement_url: String,
    #[serde(rename = "selfupdate_min_version")]
    pub selfupdate_min_version: String,
    #[serde(rename = "download_info")]
    pub download_info: serde_json::Value,
}

/// Check for available updates.
pub struct VersionChecker {
    http: reqwest::Client,
    channel: String,
    current_version: String,
}

impl VersionChecker {
    pub fn new(channel: &str, current_version: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            channel: channel.to_owned(),
            current_version: current_version.to_owned(),
        }
    }

    /// Fetch latest version info from the update server.
    pub async fn check_update(&self) -> Result<Option<VersionInfo>, super::UpdaterError> {
        let url = VERSION_URL_TEMPLATE.replace("{channel}", &self.channel);
        let info: VersionInfo = self.http.get(&url).send().await?.json().await?;
        if info.version != self.current_version {
            Ok(Some(info))
        } else {
            Ok(None)
        }
    }
}

/// Parse a semantic version string like "v0.107.0".
pub fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let v = version.trim_start_matches('v');
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() < 3 {
        return None;
    }
    let major = parts[0].parse().ok()?;
    let minor = parts[1].parse().ok()?;
    let patch = parts[2].parse().ok()?;
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("v0.107.28"), Some((0, 107, 28)));
        assert_eq!(parse_version("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_version("invalid"), None);
    }

    #[test]
    fn test_version_checker_new() {
        let checker = VersionChecker::new("release", "v0.107.28");
        assert_eq!(checker.channel, "release");
        assert_eq!(checker.current_version, "v0.107.28");
    }
}
