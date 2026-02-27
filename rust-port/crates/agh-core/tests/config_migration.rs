//! Config migration tests — TASK-45.
//!
//! Verifies that real `AdGuardHome.yaml` files (from Go deployments) are parsed
//! correctly by the Rust `AdGuardHomeConfig` deserializer with zero data loss.

use agh_core::config::AdGuardHomeConfig;

// ── Minimal config ────────────────────────────────────────────────────────────

#[test]
fn minimal_config_parses_successfully() {
    let yaml = include_str!("../../../tests/fixtures/configs/minimal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse minimal config");
    assert_eq!(config.schema_version, 28);
    assert_eq!(config.http.address, "0.0.0.0:3000");
    assert_eq!(config.dns.port, 53);
}

#[test]
fn minimal_config_roundtrip_preserves_schema_version() {
    let yaml = include_str!("../../../tests/fixtures/configs/minimal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    let roundtrip = serde_yaml::to_string(&config).unwrap();
    let config2: AdGuardHomeConfig = serde_yaml::from_str(&roundtrip).unwrap();
    assert_eq!(config.schema_version, config2.schema_version);
    assert_eq!(config.http.address, config2.http.address);
    assert_eq!(config.dns.port, config2.dns.port);
}

// ── Typical config ────────────────────────────────────────────────────────────

#[test]
fn typical_config_parses_successfully() {
    let yaml = include_str!("../../../tests/fixtures/configs/typical.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse typical config");
    assert_eq!(config.schema_version, 28);
    assert!(!config.users.is_empty(), "Expected at least one user");
    assert!(
        !config.dns.upstream_dns.is_empty(),
        "Expected upstream DNS servers"
    );
    assert_eq!(config.dns.filtering_enabled, true);
}

#[test]
fn typical_config_filters_preserved() {
    let yaml = include_str!("../../../tests/fixtures/configs/typical.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    assert!(
        !config.filters.is_empty(),
        "Expected at least one filter list"
    );
    assert_eq!(config.filters[0].enabled, true);
    assert!(!config.filters[0].url.is_empty());
}

#[test]
fn typical_config_user_rules_preserved() {
    let yaml = include_str!("../../../tests/fixtures/configs/typical.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    assert!(!config.user_rules.is_empty(), "Expected user rules");
}

#[test]
fn typical_config_roundtrip_preserves_all_key_fields() {
    let yaml = include_str!("../../../tests/fixtures/configs/typical.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    let roundtrip = serde_yaml::to_string(&config).unwrap();
    let config2: AdGuardHomeConfig = serde_yaml::from_str(&roundtrip).unwrap();
    assert_eq!(config.schema_version, config2.schema_version);
    assert_eq!(config.dns.upstream_dns, config2.dns.upstream_dns);
    assert_eq!(config.dns.filtering_enabled, config2.dns.filtering_enabled);
    assert_eq!(config.users.len(), config2.users.len());
    assert_eq!(config.filters.len(), config2.filters.len());
    assert_eq!(config.user_rules, config2.user_rules);
}

// ── Maximal config ────────────────────────────────────────────────────────────

#[test]
fn maximal_config_parses_successfully() {
    let yaml = include_str!("../../../tests/fixtures/configs/maximal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse maximal config");
    assert_eq!(config.schema_version, 28);
    assert!(config.tls.enabled);
    assert_eq!(config.tls.server_name, "home.example.com");
    assert!(config.dhcp.enabled);
    assert_eq!(config.dns.rewrites.len(), 2);
}

#[test]
fn maximal_config_multiple_filters_preserved() {
    let yaml = include_str!("../../../tests/fixtures/configs/maximal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    // 3 block filters + 1 allow filter.
    assert_eq!(config.filters.len(), 3);
    assert_eq!(config.whitelist_filters.len(), 1);
}

#[test]
fn maximal_config_dns_rewrites_preserved() {
    let yaml = include_str!("../../../tests/fixtures/configs/maximal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.dns.rewrites.len(), 2);
    assert_eq!(config.dns.rewrites[0].domain, "router.home.arpa");
    assert_eq!(config.dns.rewrites[0].answer, "192.168.1.1");
}

#[test]
fn maximal_config_statistics_and_querylog_preserved() {
    let yaml = include_str!("../../../tests/fixtures/configs/maximal.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    assert!(config.statistics.enabled);
    assert_eq!(config.statistics.interval, "168h");
    assert!(config.querylog.enabled);
    assert_eq!(config.querylog.anonymize_client_ip, true);
}

// ── Schema v27 config (older version) ────────────────────────────────────────

#[test]
fn schema27_config_parses_without_error() {
    let yaml = include_str!("../../../tests/fixtures/configs/schema27.yaml");
    // An older schema_version must not cause a parse error.
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse schema v27 config");
    assert_eq!(config.schema_version, 27);
    assert!(!config.users.is_empty());
}

#[test]
fn schema27_config_missing_fields_use_defaults() {
    let yaml = include_str!("../../../tests/fixtures/configs/schema27.yaml");
    let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
    // Fields not present in the YAML should use their serde defaults.
    assert_eq!(config.auth_attempts, 5);
    assert_eq!(config.block_auth_min, 15);
    assert!(!config.tls.enabled, "TLS should default to disabled");
    assert!(!config.dhcp.enabled, "DHCP should default to disabled");
}

// ── Unknown fields do not cause panics ───────────────────────────────────────

#[test]
fn unknown_fields_in_config_do_not_cause_parse_error() {
    let yaml = r"
schema_version: 28
http:
  address: 127.0.0.1:3000
  unknown_future_field: some_value
dns:
  bind_hosts:
    - 127.0.0.1
  port: 53
  a_field_from_future_version: true
";
    // serde_yaml ignores unknown fields by default (no #[serde(deny_unknown_fields)]).
    let result: Result<AdGuardHomeConfig, _> = serde_yaml::from_str(yaml);
    assert!(
        result.is_ok(),
        "Unknown fields should not cause parse errors"
    );
}
