# TASK-00: Cargo Workspace Scaffold

## Status
⬜ TODO

## Phase
Phase 0 — Scaffold

## Dependencies
None — this is the first task.

## Objective
Create the Cargo workspace root `Cargo.toml` with all workspace dependencies pinned. This is the foundation every subsequent crate builds upon.

## Context
The Rust workspace lives in `rust-port/`. The final binary must be a drop-in replacement for the Go `AdGuardHome` binary. See `RULES.md` for structure and `init.md` for the full overview.

---

## Checklist

- [ ] Create `rust-port/Cargo.toml` as a workspace manifest with:
  - [ ] `[workspace]` section listing all crate members:
    ```
    members = [
      "crates/agh-core",
      "crates/agh-filtering",
      "crates/agh-dns",
      "crates/agh-dhcp",
      "crates/agh-stats",
      "crates/agh-querylog",
      "crates/agh-web",
      "crates/agh-updater",
      "crates/agh-main",
    ]
    ```
  - [ ] `[workspace.dependencies]` with exact pinned versions:
    ```toml
    tokio            = { version = "1", features = ["full"] }
    axum             = "0.7"
    tower            = "0.4"
    tower-http       = { version = "0.5", features = ["fs", "cors", "trace"] }
    serde            = { version = "1", features = ["derive"] }
    serde_json       = "1"
    serde_yaml       = "0.9"
    hickory-server   = "0.24"
    hickory-resolver = "0.24"
    hickory-proto    = "0.24"
    rustls           = "0.23"
    tokio-rustls     = "0.26"
    redb             = "2"
    rust-embed       = "8"
    tracing          = "0.1"
    tracing-subscriber = { version = "0.3", features = ["env-filter"] }
    clap             = { version = "4", features = ["derive"] }
    reqwest          = { version = "0.12", features = ["rustls-tls", "json"] }
    thiserror        = "1"
    anyhow           = "1"
    notify           = "6"
    socket2          = "0.5"
    quinn            = "0.11"
    flate2           = "1"
    tar              = "0.4"
    sha2             = "0.10"
    hex              = "0.4"
    chrono           = { version = "0.4", features = ["serde"] }
    uuid             = { version = "1", features = ["v4"] }
    async-trait      = "0.1"
    bytes            = "1"
    aho-corasick     = "1"
    ```
  - [ ] `[profile.release]` with:
    ```toml
    [profile.release]
    strip     = true
    opt-level = 3
    lto       = "thin"
    codegen-units = 1
    ```
- [ ] Verify: `cd rust-port && cargo check` produces no errors (empty workspace is fine)
- [ ] Create `rust-port/.gitignore` with `target/`

---

## Verification
```bash
cd /Users/hoangnc/priv/AdGuardHome/rust-port
cargo check
```
Expected: `Finished` with 0 errors.

---

## Output Files
- `rust-port/Cargo.toml`
- `rust-port/.gitignore`
- Update `PROGRESS.md`: TASK-00 → ✅ DONE
