# TASK-01: Create All Crate Skeletons

## Status
⬜ TODO

## Phase
Phase 0 — Scaffold

## Dependencies
- TASK-00 ✅ (workspace `Cargo.toml` must exist)

## Objective
Create the directory structure and minimal `Cargo.toml` + `src/lib.rs` (or `src/main.rs`) for every crate in the workspace. Each crate must compile successfully with `cargo check -p <crate>`.

---

## Checklist

For **each** of the following crates, create the files listed:

### `crates/agh-core/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-core"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  serde      = { workspace = true }
  serde_json = { workspace = true }
  serde_yaml = { workspace = true }
  clap       = { workspace = true }
  thiserror  = { workspace = true }
  tracing    = { workspace = true }
  tokio      = { workspace = true }
  ```
- [ ] `src/lib.rs`: empty module structure with `pub mod config; pub mod client;`
- [ ] `src/config.rs`: single empty stub struct `pub struct AdGuardHomeConfig {}`
- [ ] `src/client.rs`: single empty stub struct `pub struct ClientRegistry {}`

### `crates/agh-filtering/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-filtering"
  edition = "2024"

  [dependencies]
  agh-core      = { path = "../agh-core" }
  serde         = { workspace = true }
  tokio         = { workspace = true }
  thiserror     = { workspace = true }
  tracing       = { workspace = true }
  aho-corasick  = { workspace = true }
  notify        = { workspace = true }
  reqwest       = { workspace = true }
  ```
- [ ] `src/lib.rs`: `pub mod parser; pub mod matcher; pub mod safebrowsing; pub mod safesearch; pub mod updater;`
- [ ] Stub files for each module

### `crates/agh-dns/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-dns"
  edition = "2024"

  [dependencies]
  agh-core       = { path = "../agh-core" }
  agh-filtering  = { path = "../agh-filtering" }
  hickory-server   = { workspace = true }
  hickory-resolver = { workspace = true }
  hickory-proto    = { workspace = true }
  tokio          = { workspace = true }
  rustls         = { workspace = true }
  tokio-rustls   = { workspace = true }
  axum           = { workspace = true }
  thiserror      = { workspace = true }
  tracing        = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `server`, `upstream`, `cache`, `doh`, `dot`, `doq`, `dnscrypt`

### `crates/agh-dhcp/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-dhcp"
  edition = "2024"

  [dependencies]
  agh-core   = { path = "../agh-core" }
  socket2    = { workspace = true }
  tokio      = { workspace = true }
  serde      = { workspace = true }
  serde_json = { workspace = true }
  thiserror  = { workspace = true }
  tracing    = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `v4`, `v6`, `leases`, `arp`

### `crates/agh-stats/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-stats"
  edition = "2024"

  [dependencies]
  agh-core   = { path = "../agh-core" }
  redb       = { workspace = true }
  tokio      = { workspace = true }
  serde      = { workspace = true }
  serde_json = { workspace = true }
  chrono     = { workspace = true }
  thiserror  = { workspace = true }
  tracing    = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `storage`, `aggregation`

### `crates/agh-querylog/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-querylog"
  edition = "2024"

  [dependencies]
  agh-core   = { path = "../agh-core" }
  tokio      = { workspace = true }
  serde      = { workspace = true }
  serde_json = { workspace = true }
  chrono     = { workspace = true }
  thiserror  = { workspace = true }
  tracing    = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `storage`, `query`

### `crates/agh-web/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-web"
  edition = "2024"

  [dependencies]
  agh-core      = { path = "../agh-core" }
  agh-filtering = { path = "../agh-filtering" }
  agh-dns       = { path = "../agh-dns" }
  agh-dhcp      = { path = "../agh-dhcp" }
  agh-stats     = { path = "../agh-stats" }
  agh-querylog  = { path = "../agh-querylog" }
  agh-updater   = { path = "../agh-updater" }
  axum          = { workspace = true }
  tower         = { workspace = true }
  tower-http    = { workspace = true }
  tokio         = { workspace = true }
  rust-embed    = { workspace = true }
  redb          = { workspace = true }
  serde         = { workspace = true }
  serde_json    = { workspace = true }
  thiserror     = { workspace = true }
  tracing       = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `auth`, `frontend`, `routes`

### `crates/agh-updater/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-updater"
  edition = "2024"

  [dependencies]
  agh-core  = { path = "../agh-core" }
  reqwest   = { workspace = true }
  flate2    = { workspace = true }
  tar       = { workspace = true }
  sha2      = { workspace = true }
  hex       = { workspace = true }
  tokio     = { workspace = true }
  serde     = { workspace = true }
  serde_json = { workspace = true }
  thiserror = { workspace = true }
  tracing   = { workspace = true }
  ```
- [ ] `src/lib.rs` with module stubs: `version`, `download`

### `crates/agh-main/`
- [ ] `Cargo.toml`:
  ```toml
  [package]
  name = "agh-main"
  edition = "2024"

  [[bin]]
  name = "adguardhome"
  path = "src/main.rs"

  [dependencies]
  agh-core      = { path = "../agh-core" }
  agh-filtering = { path = "../agh-filtering" }
  agh-dns       = { path = "../agh-dns" }
  agh-dhcp      = { path = "../agh-dhcp" }
  agh-stats     = { path = "../agh-stats" }
  agh-querylog  = { path = "../agh-querylog" }
  agh-web       = { path = "../agh-web" }
  agh-updater   = { path = "../agh-updater" }
  tokio         = { workspace = true }
  anyhow        = { workspace = true }
  tracing       = { workspace = true }
  tracing-subscriber = { workspace = true }
  ```
- [ ] `src/main.rs`:
  ```rust
  #[tokio::main]
  async fn main() -> anyhow::Result<()> {
      tracing_subscriber::fmt::init();
      tracing::info!("AdGuardHome starting (Rust port)");
      Ok(())
  }
  ```

---

## Verification
```bash
cd /Users/hoangnc/priv/AdGuardHome/rust-port
cargo check --workspace
cargo clippy --workspace -- -D warnings
```
All crates must compile with zero errors and zero clippy warnings.

---

## Output Files
- All 9 crate directories under `rust-port/crates/`
- Update `PROGRESS.md`: TASK-01 → ✅ DONE
