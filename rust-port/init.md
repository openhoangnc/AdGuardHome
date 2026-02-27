# Detailed Plan: Porting AdGuard Home Backend to Rust

This is a large-scale rewrite project. The Go backend is ~62.5% of the codebase, covering DNS resolution, DHCP, filtering, HTTP API, authentication, stats, query logging, TLS, and auto-update. The frontend (React/TypeScript) must remain **100% untouched** — the Rust backend must expose identical REST API contracts.

***

## Project Overview

| Aspect | Current (Go) | Target (Rust) |
|---|---|---|
| Language | Go 1.25+ | Rust 2026 (1.93) edition |
| DNS library | `dnsproxy`, `miekg/dns` | `hickory-dns` (formerly trust-dns) |
| HTTP server | `net/http` (stdlib) | `axum` + `tokio` |
| Config format | YAML (`AdGuardHome.yaml`) | Same YAML, parsed with `serde_yaml` |
| Frontend serving | Embedded `embed.FS` | Embedded with `rust-embed` or `include_dir` |
| DHCP | `dhcpd` package | `dhcrs` or custom via raw sockets |
| TLS | stdlib `crypto/tls` | `rustls` |
| Auth sessions | BoltDB (`sessions.db`) | `redb` or `sled` (embedded KV) |
| Docker multi-arch | Buildx + QEMU | Same Buildx + `cross-rs` |

***

## Phase 1 — Preparation & API Contract Audit (Week 1–2)

Before writing a single line of Rust, lock down the API contract so the frontend never breaks.

1. **Export full OpenAPI spec** from `openapi/openapi.yaml` in the repo — this is the ground truth for all ~80+ REST endpoints.
2. **Map every HTTP route** registered in Go (`internal/home/*.go`, `internal/dnsforward/`, `internal/dhcpd/`, `internal/filtering/`, `internal/querylog/`, `internal/stats/`) to its JSON schema.
3. **Document all static file serving** — the backend serves the frontend build at `/` and API at `/control/` prefix.
4. **Capture WebSocket/SSE usage** — AdGuard Home uses `GET /control/querylog` with streaming; replicate this exactly.
5. **Record config file schema** (`AdGuardHome.yaml`) with all fields — the Rust binary must read/write the exact same YAML structure for zero-migration upgrades.
6. **Set up integration test harness** using the existing frontend E2E Playwright tests against a mock backend to baseline expected behavior.

***

## Phase 2 — Rust Project Scaffold (Week 2–3)

```
adguardhome-rust/
├── Cargo.toml              # workspace
├── crates/
│   ├── agh-core/           # shared types, config structs (serde)
│   ├── agh-dns/            # DNS server + filtering engine
│   ├── agh-dhcp/           # DHCP server
│   ├── agh-web/            # HTTP API (axum), auth, TLS
│   ├── agh-stats/          # statistics storage
│   ├── agh-querylog/       # query log storage
│   ├── agh-filtering/      # blocklist engine, safesearch, parental
│   ├── agh-updater/        # self-update module
│   └── agh-main/           # binary entrypoint, wires everything
├── build/                  # frontend build output (unchanged)
├── Dockerfile
├── docker-buildx.sh
└── AdGuardHome.yaml        # config (identical schema)
```

**Key Cargo dependencies:**

```toml
[workspace.dependencies]
tokio          = { version = "1.49", features = ["full"] }
axum           = "0.8"
tower          = "0.5"
serde          = { version = "1.0", features = ["derive"] }
serde_json     = "1.0"
serde_yaml     = "0.9"
hickory-server = "0.25"
hickory-resolver = "0.25"
rustls         = "0.23"
tokio-rustls   = "0.26"
redb           = "3.1"           # session DB (replaces BoltDB)
rust-embed     = "8.11"           # embed frontend build/
tracing        = "0.1"
tracing-subscriber = "0.3"
clap           = { version = "4.5", features = ["derive"] }
reqwest        = { version = "0.13", features = ["rustls-tls"] }
sled           = "0.34"        # optional KV for stats/querylog
```

***

## Phase 3 — Core Modules Implementation (Week 3–10)

Implement in dependency order — each crate must have unit tests before the next starts.

### 3.1 `agh-core` — Config & Shared Types (Week 3)
- Implement `AdGuardHomeConfig` struct with `serde` matching **exact YAML field names** from Go (`dns`, `tls`, `filters`, `clients`, `http`, `os`, etc.)
- Implement config read/write with atomic file writes (write to `.tmp`, rename)
- Implement CLI argument parsing with `clap` — match all Go flags: `--config`, `--work-dir`, `--host`, `--port`, `--no-etc-hosts`, `--local-frontend`, `--service`
- Implement `firstRun` detection (check if config file exists)

### 3.2 `agh-filtering` — DNS Filtering Engine (Week 4–5)
- Parse AdBlock/hosts-format blocklists (port logic from `internal/filtering/`)
- Implement rule matching: exact domain, wildcard `*.example.com`, regex rules
- Implement safe browsing via hash-prefix DNS lookup (`sb.dns.adguard.com`)
- Implement parental control hash-prefix lookup (`pc.dns.adguard.com`)
- Implement safe search rewrites (Google, Bing, DuckDuckGo, YouTube)
- Implement `/etc/hosts` file watching with `notify` crate (replaces `aghos.NewOSWritesWatcher`)
- Blocklist auto-update scheduler with `tokio` timers

### 3.3 `agh-dns` — DNS Server (Week 5–7)
- Build DNS server on `hickory-server` listening on UDP/TCP port 53
- Implement DNS-over-HTTPS (DoH) server via axum handler at `GET/POST /dns-query`
- Implement DNS-over-TLS (DoT) with `tokio-rustls`
- Implement DNS-over-QUIC (DoQ) with `quinn`
- Implement DNSCrypt with `dnscrypt` crate or custom implementation
- Implement upstream resolver selection (plain DNS, DoH, DoT, DoQ upstreams)
- Implement parallel upstream queries and fastest-response selection
- Wire filtering engine: intercept queries → check blocklist → return `0.0.0.0` or NXDOMAIN
- Implement per-client rules lookup (calls `agh-core` client registry)
- Implement EDNS Client Subnet handling
- Implement DNS cache with TTL

### 3.4 `agh-dhcp` — DHCP Server (Week 6–7)
- Implement DHCPv4 server (RFC 2131) using raw UDP sockets via `socket2`
- Implement DHCPv6 server (RFC 3315)
- Lease storage in YAML/JSON file (same schema as Go's `leases.db`)
- ARP table reading for client discovery — Linux (`/proc/net/arp`), macOS (`arp -an`)
- HTTP API handlers for DHCP (registered in `agh-web`)

### 3.5 `agh-stats` — Statistics (Week 7)
- Circular buffer time-series stats stored in `redb` (matches Go's `internal/stats`)
- Schema: queries per period, blocked queries, top clients, top domains, top blocked
- Configurable retention window (1h, 24h, 7d, 30d, 90d)
- JSON serialization matching Go's `GET /control/stats` response exactly

### 3.6 `agh-querylog` — Query Log (Week 7–8)
- Append-only log with configurable size limit and retention
- Persistent storage in JSON-lines file (matching Go's `querylog.json` + `querylog.json.1`)
- Streaming response for `GET /control/querylog` with cursor-based pagination
- IP anonymization support (mask last octet)
- Filters: by client, domain, answer, blocked status

### 3.7 `agh-web` — HTTP API + Frontend (Week 8–9)
This is the most critical module — every endpoint must be API-compatible.

**Authentication:**
- Session-based auth with `sessions.db` via `redb`
- `POST /control/login`, `GET /control/logout`
- Cookie: `agh_session` (same name as Go)
- Rate limiting on failed logins (matches `authRateLimiter`)
- Trusted proxies support for `X-Forwarded-For`

**Frontend serving:**
```rust
// Embed the prebuilt frontend (build/ directory — never modified)
#[derive(RustEmbed)]
#[folder = "build/static"]
struct ClientAssets;

// Serve at / with fallback to index.html for SPA routing
async fn serve_frontend(uri: Uri) -> impl IntoResponse { ... }
```

**All REST API routes (axum Router):**

| Method | Path | Module |
|---|---|---|
| `GET/POST` | `/control/install/*` | Setup wizard |
| `GET` | `/control/status` | Core status |
| `GET/POST` | `/control/dns_info` / `dns_config` | DNS settings |
| `GET/POST` | `/control/filtering/status` / `config` | Filtering |
| `POST` | `/control/filtering/add_url` | Blocklist mgmt |
| `GET` | `/control/querylog` | Query log stream |
| `GET/POST` | `/control/querylog_info` / `config` | Querylog config |
| `GET/POST` | `/control/stats` / `stats_config` | Statistics |
| `GET/POST` | `/control/clients` | Client management |
| `GET/POST` | `/control/dhcp/*` | DHCP server |
| `GET/POST` | `/control/tls/*` | TLS config |
| `GET/POST` | `/control/safebrowsing/*` | Safe browsing |
| `GET/POST` | `/control/parental/*` | Parental control |
| `GET/POST` | `/control/safesearch/*` | Safe search |
| `GET` | `/control/version.json` | Update check |
| `POST` | `/control/update` | Self-update |
| `GET/POST` | `/control/access/*` | Access control |
| `GET/POST` | `/control/rewrite/*` | DNS rewrites |
| `GET/POST` | `/control/blocked_services/*` | Blocked services |
| `POST` | `/control/test_upstream_dns` | Upstream test |
| `GET` | `/control/profile` | User profile |
| `POST` | `/control/change_language` | i18n |

**JSON response validation:** Write an automated test that calls each endpoint and validates the response schema against the OpenAPI spec using `openapiv3` crate.

### 3.8 `agh-updater` — Self-Update (Week 9)
- Fetch version info from `https://static.adguard.com/adguardhome/{channel}/version.json`
- Download, verify checksum, extract tarball with `flate2` + `tar`
- Atomic binary replacement
- Set `GOARCH`/`GOOS` equivalent: detect at compile time via `cfg!(target_arch)` and `cfg!(target_os)`

***

## Phase 4 — Service Management (Week 9)

The Go backend uses the `service` library to install as a system service. Replicate with:
- **Linux systemd**: generate `.service` unit file, call `systemctl enable/start/stop`
- **macOS launchd**: generate `.plist`, call `launchctl`
- **Windows SCM**: use `windows-service` crate
- CLI subcommands: `--service install|uninstall|start|stop|restart|status` (identical to Go)

***

## Phase 5 — Multi-Arch Dockerfile (Week 10)

The current AdGuard Home uses Docker Buildx with QEMU for multi-arch. Replicate this pattern with Rust's `cross` toolchain:

```dockerfile
# ── Stage 1: Build frontend (unchanged, Node.js) ──────────────────────────
FROM node:24-alpine AS frontend-builder
WORKDIR /app
COPY client/ ./client/
COPY Makefile ./
RUN cd client && npm ci && npm run build

# ── Stage 2: Build Rust backend (multi-arch via cross-compilation) ────────
FROM --platform=$BUILDPLATFORM rust:1.82-alpine AS rust-builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Install cross-compilation toolchains
RUN apk add --no-cache musl-dev gcc g++ make perl

# Install cross-rs for cross-compilation
RUN cargo install cross --git https://github.com/cross-rs/cross

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Map Docker TARGETPLATFORM to Rust target triple
RUN case "$TARGETPLATFORM" in \
  "linux/amd64")   RUST_TARGET="x86_64-unknown-linux-musl" ;; \
  "linux/arm64")   RUST_TARGET="aarch64-unknown-linux-musl" ;; \
  "linux/arm/v7")  RUST_TARGET="armv7-unknown-linux-musleabihf" ;; \
  "linux/arm/v6")  RUST_TARGET="arm-unknown-linux-musleabihf" ;; \
  "linux/386")     RUST_TARGET="i686-unknown-linux-musl" ;; \
  "linux/ppc64le") RUST_TARGET="powerpc64le-unknown-linux-gnu" ;; \
  "linux/s390x")   RUST_TARGET="s390x-unknown-linux-gnu" ;; \
  *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
  esac && \
  rustup target add $RUST_TARGET && \
  cargo build --release --target $RUST_TARGET \
    --bin adguardhome && \
  cp target/$RUST_TARGET/release/adguardhome /adguardhome

# ── Stage 3: Final minimal image ─────────────────────────────────────────
FROM alpine:3.20
RUN apk add --no-cache ca-certificates tzdata libcap && \
    setcap 'cap_net_bind_service=+ep' /opt/adguardhome/AdGuardHome

COPY --from=rust-builder /adguardhome /opt/adguardhome/AdGuardHome
COPY --from=frontend-builder /app/build /opt/adguardhome/build

VOLUME ["/opt/adguardhome/conf", "/opt/adguardhome/work"]
EXPOSE 53/tcp 53/udp 67/udp 68/udp 80/tcp 443/tcp 443/udp 3000/tcp 853/tcp 853/udp 5443/tcp 5443/udp 6060/tcp

ENTRYPOINT ["/opt/adguardhome/AdGuardHome"]
CMD ["-c", "/opt/adguardhome/conf/AdGuardHome.yaml", \
     "-w", "/opt/adguardhome/work", "--no-check-update"]
```

**Build & push multi-arch manifest:**

```bash
docker buildx create --name agh-builder --driver docker-container --use
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

docker buildx build \
  --platform linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/386,linux/ppc64le,linux/s390x \
  -t yourrepo/adguardhome-rust:latest \
  --push \
  .
```

***

## Phase 6 — Testing & Validation (Week 10–12)

### Compatibility Testing Strategy
1. **API contract tests**: Run the existing Playwright E2E frontend tests against the Rust backend — all must pass with zero frontend changes.
2. **Config migration test**: Take real `AdGuardHome.yaml` files from production Go deployments and verify the Rust binary reads/writes them identically.
3. **DNS compliance tests**: Use `dnscompliance` and `q` tool to test DoH/DoT/DoQ/DNSCrypt endpoints.
4. **DHCP tests**: Test lease assignment, renewal, and release in a virtual network.
5. **Regression test**: Run both Go and Rust binaries side-by-side, compare JSON responses for every endpoint.
6. **Performance benchmarks**: Use `k6` to compare request throughput and DNS query latency — Rust should be ≥2x better on DNS throughput due to zero-GC.

***

## Phase 7 — CI/CD Pipeline

```yaml
# .github/workflows/build.yml
name: Build Multi-Arch
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: docker/setup-qemu-action@v3
      - uses: docker/setup-buildx-action@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run tests
        run: cargo test --workspace
      - name: Build & push Docker
        uses: docker/build-push-action@v6
        with:
          platforms: linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/386,linux/ppc64le,linux/s390x
          push: ${{ github.ref == 'refs/heads/main' }}
          tags: yourrepo/adguardhome-rust:latest
```

***

## Critical Constraints & Risks

| Risk | Mitigation |
|---|---|
| `hickory-dns` DoQ/DNSCrypt gaps | Contribute to hickory or wrap C library via FFI (`libdnscrypt`) |
| DHCP raw socket permissions on non-Linux | Abstract behind trait; use platform-specific backends |
| `sessions.db` BoltDB → `redb` migration | Write one-time migration tool; detect and convert on first run |
| Windows service binary size | Use `strip`, `opt-level = "z"`, `lto = true` in Cargo.toml release profile |
| Blocking list performance at scale (millions of rules) | Use `aho-corasick` + radix trie instead of linear scan |
| Frontend assumes specific CORS/cookie behavior | Audit and replicate exact `Set-Cookie` flags and CORS headers from Go `net/http` |

***

## Timeline Summary

| Week | Milestone |
|---|---|
| 1–2 | API audit, OpenAPI locked, test harness |
| 2–3 | Cargo workspace scaffold, `agh-core` config |
| 4–5 | `agh-filtering` engine fully tested |
| 5–7 | `agh-dns` (plain + DoH/DoT/DoQ/DNSCrypt) |
| 6–7 | `agh-dhcp` |
| 7–8 | `agh-stats` + `agh-querylog` |
| 8–9 | `agh-web` all API routes, auth, frontend serving |
| 9 | `agh-updater` + service management |
| 10 | Dockerfile multi-arch, CI pipeline |
| 10–12 | Full E2E validation, performance benchmarks |

The most important rule throughout: **never modify anything under `client/`**. The Rust binary embeds the prebuilt `build/` output exactly as the Go binary does, and serves it via `rust-embed` at the same paths. [github](https://github.com/AdguardTeam/AdGuardHome/blob/master/internal/home/home.go)
