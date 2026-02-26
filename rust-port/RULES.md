# Agent Rules — AdGuardHome Rust Port

> These rules are **mandatory** for every Agent working on this project. Read them before touching any file.

---

## 🔴 ABSOLUTE CONSTRAINTS

1. **Never modify anything under `client/`** — the frontend is completely off-limits.
2. **Never change `openapi/openapi.yaml`** — it is the ground truth API contract. Read it, do not write to it.
3. **Never change `AdGuardHome.yaml` schema** — the Rust binary must read/write the exact same YAML as the Go binary. Zero migration required for existing users.
4. **Every crate must compile with `cargo check` before marking a task done.**
5. **Every crate must have unit tests (`cargo test -p <crate>`) before the next phase begins.**
6. **All HTTP responses must match the OpenAPI spec exactly** — field names, types, HTTP status codes, headers (especially `Set-Cookie: agh_session`, CORS headers).
7. **Use only workspace dependencies** declared in the root `Cargo.toml` — no crate-local version pins except for dev-only dependencies.

---

## 📁 Project Structure (canonical)

```
adguardhome-rust/          ← Rust workspace root (rust-port/ maps here)
├── Cargo.toml             # workspace manifest
├── crates/
│   ├── agh-core/          # shared types, config, CLI
│   ├── agh-filtering/     # blocklist engine, safe browsing
│   ├── agh-dns/           # DNS server (hickory-server)
│   ├── agh-dhcp/          # DHCPv4/v6 server
│   ├── agh-stats/         # circular stats storage
│   ├── agh-querylog/      # query log append storage
│   ├── agh-web/           # axum HTTP API + frontend serving
│   ├── agh-updater/       # self-update module
│   └── agh-main/          # binary entrypoint
├── build/                 # frontend build output (NEVER TOUCH)
└── AdGuardHome.yaml
```

---

## 🔧 Coding Standards

- **Edition**: Rust 2024 (`edition = "2024"` in every `Cargo.toml`)
- **Async runtime**: `tokio` with `features = ["full"]` — no blocking calls on async threads
- **Error handling**: Use `thiserror` for library crates, `anyhow` for binary crates
- **Logging**: `tracing` macros only — no `println!` or `eprintln!` in library code
- **Serialization**: `serde` with `#[serde(rename_all = "camelCase")]` where the Go API uses camelCase JSON, `#[serde(rename = "field_name")]` for YAML config fields that use snake_case
- **Config atomicity**: Always write config to a `.tmp` file then `rename()` — never write directly to the target
- **No `unwrap()`**: Use `?` propagation or `expect("descriptive message")` in tests only
- **Clippy**: Code must pass `cargo clippy -- -D warnings` before marking done

---

## 📋 Task File Protocol

Each task file lives at `rust-port/tasks/TASK-XX-name.md`.

When starting a task:
1. Open the task file.
2. Update `## Status` to `🟡 IN PROGRESS` and add the start date.
3. Work through each checklist item.
4. When done, update `## Status` to `✅ DONE` and update `rust-port/PROGRESS.md`.

When a task is blocked:
- Update `## Status` to `🔴 BLOCKED` with a clear reason.
- Update `PROGRESS.md` with the blocker.

---

## 🧪 Test Requirements per Crate

| Crate | Minimum Test Coverage |
|---|---|
| `agh-core` | Config round-trip (read→write→read), CLI flag parsing |
| `agh-filtering` | Blocklist parse, rule match, safe search rewrite |
| `agh-dns` | DNS query resolution, cache hit, block response |
| `agh-dhcp` | DHCP DISCOVER/OFFER/REQUEST/ACK cycle |
| `agh-stats` | Write stats, read back, retention expiry |
| `agh-querylog` | Append entry, paginated read, filter by client |
| `agh-web` | Every REST endpoint returns correct HTTP status; auth cookie set correctly |
| `agh-updater` | Version parse, checksum verify (mock download) |

---

## 🔗 Dependency Order

Tasks **must** be completed in this order (later crates depend on earlier ones):

```
agh-core → agh-filtering → agh-dns
                         → agh-dhcp
         → agh-stats
         → agh-querylog
         → agh-web (depends on all above)
         → agh-updater
         → agh-main (wires all)
```

Scaffold tasks (workspace `Cargo.toml`, crate skeletons) happen first, before any implementation.

---

## 📌 Reference Files

| File | Purpose |
|---|---|
| `openapi/openapi.yaml` | API contract — every endpoint lives here |
| `internal/home/home.go` | Go HTTP server route registration |
| `internal/filtering/filtering.go` | Filtering logic to port |
| `internal/dnsforward/dnsforward.go` | DNS server logic to port |
| `internal/stats/stats.go` | Stats module to port |
| `internal/querylog/querylog.go` | Query log to port |
| `internal/dhcpd/dhcpd.go` | DHCP module to port |
| `AGHTechDoc.md` | Architecture documentation |
