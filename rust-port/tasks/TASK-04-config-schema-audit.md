# TASK-04: Config YAML Schema Audit

## Status
⬜ TODO

## Phase
Phase 1 — Preparation

## Dependencies
None (can run in parallel with TASK-00).

## Objective
Extract the complete `AdGuardHome.yaml` schema from the Go source and produce a reference document. This drives the `serde` field mapping in TASK-06.

---

## Checklist

- [ ] Read the Go config structs in:
  - `internal/home/config.go` — root `configuration` struct
  - `internal/dnsforward/config.go` — `DNSConfig` struct
  - `internal/filtering/config.go` — `FilterConfig` struct
  - `internal/dhcpd/config.go` — `DhcpConfig` struct
  - `internal/querylog/config.go` — query log config
  - `internal/stats/config.go` — stats config
  - `internal/home/tls.go` — TLS config
- [ ] Note each struct field's **Go field name**, **YAML tag** (the `yaml:"..."` annotation, not the Go name), **type**, and **default value**
- [ ] Produce `rust-port/docs/config-schema.md` with a table:
  | YAML key | Go type | Rust type | Default | Notes |
  |---|---|---|---|---|
- [ ] Section the table by top-level config block: `dns`, `tls`, `filters`, `clients`, `http`, `os`, `schema_version`, `auth_attempts`, `block_auth_min`, `users`, etc.
- [ ] Find a real `AdGuardHome.yaml` example in the repo (check `scripts/`, `snap/`, `docker/`) and verify the table is complete against it
- [ ] Identify any fields that use **non-standard YAML types** (duration strings, IP addresses as strings, etc.) and note the required custom `serde` deserializers

---

## Key Files to Read

```
internal/home/config.go
internal/dnsforward/config.go
internal/filtering/config.go
internal/dhcpd/config.go
internal/querylog/config.go
internal/stats/config.go
internal/home/tls.go
snap/local/adguardhome.yaml   (or similar example config)
docker/   (check for example YAML)
```

---

## Verification
Run `grep -c "yaml:" internal/home/config.go` to count expected fields, and verify the output table has at least that many rows. The Rust `agh-core` config struct (TASK-06) must roundtrip a sample YAML with zero field loss.

---

## Output Files
- `rust-port/docs/config-schema.md`
- Update `PROGRESS.md`: TASK-04 → ✅ DONE
