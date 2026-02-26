# TASK-45: Config Migration Tests

## Status
⬜ TODO

## Phase
Phase 12 — Testing

## Dependencies
- TASK-07 ✅ (ConfigManager read/write)

## Objective
Verify that real `AdGuardHome.yaml` files from Go deployments are read correctly by the Rust binary with zero data loss — the most critical compatibility requirement.

---

## Checklist

- [ ] Collect sample config files:
  - [ ] `rust-port/tests/fixtures/configs/minimal.yaml` — just `schema_version` and `http`
  - [ ] `rust-port/tests/fixtures/configs/typical.yaml` — typical home user config (DNS, filters, one user)
  - [ ] `rust-port/tests/fixtures/configs/maximal.yaml` — all features enabled (DHCP, TLS, per-client rules, custom upstreams, all filters)
  - [ ] `rust-port/tests/fixtures/configs/schema27.yaml` — older schema version for migration testing

  > Source real configs from the AdGuardHome GitHub issues, wiki, or generate from Go binary defaults.

- [ ] Write `rust-port/tests/config_migration_test.rs`:

  ```rust
  #[test]
  fn test_minimal_config_roundtrip() {
      let yaml = include_str!("fixtures/configs/minimal.yaml");
      let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse failed");
      let roundtrip = serde_yaml::to_string(&config).unwrap();
      let config2: AdGuardHomeConfig = serde_yaml::from_str(&roundtrip).unwrap();
      // Key fields must survive roundtrip
      assert_eq!(config.schema_version, config2.schema_version);
      assert_eq!(config.http.address, config2.http.address);
  }

  #[test]
  fn test_typical_config_all_fields_preserved() {
      let yaml = include_str!("fixtures/configs/typical.yaml");
      let config: AdGuardHomeConfig = serde_yaml::from_str(yaml).expect("parse failed");
      // Verify specific key fields
      assert!(!config.dns.upstream_dns.is_empty());
      assert!(!config.users.is_empty());
  }

  #[test]
  fn test_maximal_config_no_unknown_fields() {
      // Use serde_yaml with `deny_unknown_fields` equivalent — ensure no fields are silently dropped
  }

  #[test]
  fn test_schema27_parsed_correctly() {
      // Older schema version must be accepted (no panic, no error)
  }
  ```

- [ ] Run `AdGuardHome.yaml` from the actual machine's Go deployment through the Rust `ConfigManager`:
  ```bash
  ./target/debug/adguardhome -c /path/to/real/AdGuardHome.yaml --dry-run
  ```
  (Add `--dry-run` flag to TASK-08 CLI if not present: parse config, print status, exit 0)

---

## Acceptance Criteria
- All 4 fixture configs parse without error
- Roundtrip preserves all field values
- Unknown future fields don't cause parse errors (serde handles this with `#[serde(flatten)]` or `#[serde(deny_unknown_fields)]` absent)

---

## Verification
```bash
cargo test config_migration
```

---

## Output Files
- `rust-port/tests/config_migration_test.rs`
- `rust-port/tests/fixtures/configs/*.yaml`
- Update `PROGRESS.md`: TASK-45 → ✅ DONE
