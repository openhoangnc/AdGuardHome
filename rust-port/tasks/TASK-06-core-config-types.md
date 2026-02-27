# TASK-06: `agh-core` — Config Structs & Serde

## Status
⬜ TODO

## Phase
Phase 2 — `agh-core`

## Dependencies
- TASK-01 ✅ (crate skeleton exists)
- TASK-04 ✅ (config schema audit complete — use `docs/config-schema.md`)

## Objective
Implement the complete `AdGuardHomeConfig` struct tree in `agh-core` with `serde` annotations that exactly match the YAML field names used by the Go backend. A real `AdGuardHome.yaml` from a Go deployment must deserialize without error.

---

## Checklist

- [ ] Implement `src/config.rs` with full struct tree:

  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct AdGuardHomeConfig {
      pub schema_version: u32,
      pub http: HttpConfig,
      pub users: Vec<User>,
      pub auth_attempts: u32,
      pub block_auth_min: u32,
      pub dns: DnsConfig,
      pub tls: TlsConfig,
      pub filters: Vec<FilterConfig>,
      pub whitelist_filters: Vec<FilterConfig>,
      pub user_rules: Vec<String>,
      pub dhcp: DhcpConfig,
      pub clients: ClientsConfig,
      pub log: LogConfig,
      pub os: OsConfig,
      pub statistics: StatisticsConfig,
      pub querylog: QueryLogConfig,
  }
  ```

  Each nested struct must be in its own file under `src/config/`:
  - `http.rs` → `HttpConfig`
  - `dns.rs` → `DnsConfig` (upstream servers, bootstrap, parallel, cache settings, etc.)
  - `tls.rs` → `TlsConfig`
  - `filter.rs` → `FilterConfig` (url, name, enabled, id)
  - `dhcp.rs` → `DhcpConfig`
  - `clients.rs` → `ClientsConfig` (runtime_sources, persistent client list)
  - `user.rs` → `User` (name, password hash)
  - `log.rs` → `LogConfig`
  - `os.rs` → `OsConfig`
  - `stats.rs` → `StatisticsConfig` (interval, ignored)
  - `querylog.rs` → `QueryLogConfig` (enabled, size_mb, ignored)

- [ ] Use `#[serde(rename = "yaml_key")]` wherever the YAML key differs from Rust field naming convention
- [ ] Use `#[serde(default)]` on optional fields so partial configs deserialize without error
- [ ] Use `#[serde(skip_serializing_if = "Option::is_none")]` for truly optional fields
- [ ] Add `Default` impl for all structs (matching Go defaults from `config.go`)
- [ ] Add `impl AdGuardHomeConfig { pub fn default_path() -> PathBuf }` returning `WorkDir/AdGuardHome.yaml`

---

## Tests

- [ ] `src/config.rs` tests module:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_default_config_serializes() {
          let cfg = AdGuardHomeConfig::default();
          let yaml = serde_yaml::to_string(&cfg).unwrap();
          let roundtrip: AdGuardHomeConfig = serde_yaml::from_str(&yaml).unwrap();
          assert_eq!(cfg.schema_version, roundtrip.schema_version);
      }

      #[test]
      fn test_partial_config_deserializes() {
          let yaml = "schema_version: 28\nhttp:\n  address: 0.0.0.0:3000\n";
          let cfg: AdGuardHomeConfig = serde_yaml::from_str(yaml).unwrap();
          assert_eq!(cfg.http.address, "0.0.0.0:3000");
      }
  }
  ```

---

## Verification
```bash
cd rust-port
cargo test -p agh-core
cargo clippy -p agh-core -- -D warnings
```

---

## Output Files
- `rust-port/crates/agh-core/src/config/` (all nested config structs)
- Update `PROGRESS.md`: TASK-06 → ✅ DONE
