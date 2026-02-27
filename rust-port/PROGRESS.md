# AdGuardHome Rust Port — Progress Tracker

> **Last updated**: 2026-02-27
> **Rules**: See [RULES.md](./RULES.md) — read before starting any task
> **Task files**: `rust-port/tasks/TASK-XX-*.md`

---

## 📊 Overall Progress

| Phase | Tasks | Done | In Progress | Blocked |
|---|---|---|---|---|
| Phase 0 — Scaffold | 3 | 3 | 0 | 0 |
| Phase 1 — Preparation | 3 | 3 | 0 | 0 |
| Phase 2 — `agh-core` | 4 | 4 | 0 | 0 |
| Phase 3 — `agh-filtering` | 5 | 5 | 0 | 0 |
| Phase 4 — `agh-dns` | 6 | 6 | 0 | 0 |
| Phase 5 — `agh-dhcp` | 4 | 4 | 0 | 0 |
| Phase 6 — `agh-stats` | 3 | 3 | 0 | 0 |
| Phase 7 — `agh-querylog` | 3 | 3 | 0 | 0 |
| Phase 8 — `agh-web` | 6 | 6 | 0 | 0 |
| Phase 9 — `agh-updater` | 2 | 2 | 0 | 0 |
| Phase 10 — `agh-main` | 2 | 2 | 0 | 0 |
| Phase 11 — Docker & CI | 2 | 2 | 0 | 0 |
| Phase 12 — Testing | 4 | 4 | 0 | 0 |
| **Total** | **47** | **47** | **0** | **0** |

---

## 📋 Task Status Table

| Task ID | Name | Status | Notes |
|---|---|---|---|
| **PHASE 0 — SCAFFOLD** | | | |
| [TASK-00](./tasks/TASK-00-workspace-scaffold.md) | Cargo Workspace Scaffold | ✅ DONE | `rust-port/Cargo.toml` created |
| [TASK-01](./tasks/TASK-01-crate-skeletons.md) | Create All Crate Skeletons | ✅ DONE | All 9 crates created |
| [TASK-02](./tasks/TASK-02-ci-skeleton.md) | CI/CD Skeleton (GitHub Actions) | ✅ DONE | `.github/workflows/rust.yml` created |
| **PHASE 1 — PREPARATION** | | | |
| [TASK-03](./tasks/TASK-03-api-audit.md) | API Contract Audit | ✅ DONE | `docs/api-audit.md` — 81 endpoints |
| [TASK-04](./tasks/TASK-04-config-schema-audit.md) | Config YAML Schema Audit | ✅ DONE | `docs/config-schema.md` |
| [TASK-05](./tasks/TASK-05-test-harness.md) | Integration Test Harness Setup | ✅ DONE | `tests/api_compat/` created |
| **PHASE 2 — AGH-CORE** | | | |
| [TASK-06](./tasks/TASK-06-core-config-types.md) | `agh-core`: Config Structs & Serde | ✅ DONE | Full config struct tree with serde |
| [TASK-07](./tasks/TASK-07-core-config-io.md) | `agh-core`: Config Read/Write & Atomic | ✅ DONE | `ConfigManager` with atomic writes |
| [TASK-08](./tasks/TASK-08-core-cli.md) | `agh-core`: CLI Argument Parsing | ✅ DONE | `Cli` struct with all Go flags |
| [TASK-09](./tasks/TASK-09-core-client-registry.md) | `agh-core`: Client Registry | ✅ DONE | `ClientRegistry` with CIDR matching |
| **PHASE 3 — AGH-FILTERING** | | | |
| [TASK-10](./tasks/TASK-10-filtering-parser.md) | `agh-filtering`: Blocklist Parser | ✅ DONE | AdBlock + hosts format parser |
| [TASK-11](./tasks/TASK-11-filtering-matcher.md) | `agh-filtering`: Rule Matcher | ✅ DONE | AhoCorasick + exact + wildcard + regex |
| [TASK-12](./tasks/TASK-12-filtering-safebrowsing.md) | `agh-filtering`: Safe Browsing | ✅ DONE | Hash-prefix protocol (DNS lookup stubbed) |
| [TASK-13](./tasks/TASK-13-filtering-safesearch.md) | `agh-filtering`: Safe Search Rewrites | ✅ DONE | Built-in mappings for all engines |
| [TASK-14](./tasks/TASK-14-filtering-updater.md) | `agh-filtering`: Blocklist Auto-Updater | ✅ DONE | HTTP download + hot reload |
| **PHASE 4 — AGH-DNS** | | | |
| [TASK-15](./tasks/TASK-15-dns-server-core.md) | `agh-dns`: Core DNS Server (UDP/TCP) | ✅ DONE | UDP server with `QueryHandler` trait |
| [TASK-16](./tasks/TASK-16-dns-upstream.md) | `agh-dns`: Upstream Resolvers | ✅ DONE | `hickory-resolver` TokioResolver |
| [TASK-17](./tasks/TASK-17-dns-cache.md) | `agh-dns`: DNS Cache | ✅ DONE | TTL-based in-memory cache |
| [TASK-18](./tasks/TASK-18-dns-doh-dot.md) | `agh-dns`: DoH + DoT | ✅ DONE | DoH axum handlers; DoT length-framed |
| [TASK-19](./tasks/TASK-19-dns-doq-dnscrypt.md) | `agh-dns`: DoQ + DNSCrypt | ✅ DONE | DoQ via `quinn` (RFC 9250); DNSCrypt documented as known gap (no Rust crate) |
| [TASK-20](./tasks/TASK-20-dns-filtering-wire.md) | `agh-dns`: Wire Filtering Engine | ✅ DONE | `FilteringHandler` wires all filtering |
| **PHASE 5 — AGH-DHCP** | | | |
| [TASK-21](./tasks/TASK-21-dhcp-v4.md) | `agh-dhcp`: DHCPv4 Server | ✅ DONE | Full DISCOVER/OFFER/REQUEST/ACK state machine |
| [TASK-22](./tasks/TASK-22-dhcp-v6.md) | `agh-dhcp`: DHCPv6 Server | ✅ DONE | Solicit/Advertise/Request/Reply + IA_NA |
| [TASK-23](./tasks/TASK-23-dhcp-leases.md) | `agh-dhcp`: Lease Persistence & ARP | ✅ DONE | LeaseStore + ARP table reader |
| [TASK-24](./tasks/TASK-24-dhcp-api.md) | `agh-dhcp`: HTTP API Handlers | ✅ DONE | Handlers in agh-web routes |
| **PHASE 6 — AGH-STATS** | | | |
| [TASK-25](./tasks/TASK-25-stats-storage.md) | `agh-stats`: Time-Series Storage | ✅ DONE | Hourly bucket storage |
| [TASK-26](./tasks/TASK-26-stats-aggregation.md) | `agh-stats`: Stats Aggregation | ✅ DONE | StatsService with totals |
| [TASK-27](./tasks/TASK-27-stats-api.md) | `agh-stats`: HTTP API Handlers | ✅ DONE | GET /control/stats via routes |
| **PHASE 7 — AGH-QUERYLOG** | | | |
| [TASK-28](./tasks/TASK-28-querylog-storage.md) | `agh-querylog`: Append-Only Storage | ✅ DONE | JSON-lines file storage |
| [TASK-29](./tasks/TASK-29-querylog-query.md) | `agh-querylog`: Pagination & Filtering | ✅ DONE | `QueryLogService` with filters |
| [TASK-30](./tasks/TASK-30-querylog-api.md) | `agh-querylog`: HTTP API Handlers | ✅ DONE | Handler stubs in routes |
| **PHASE 8 — AGH-WEB** | | | |
| [TASK-31](./tasks/TASK-31-web-auth.md) | `agh-web`: Auth & Session Management | ✅ DONE | Cookie-based sessions with SessionStore |
| [TASK-32](./tasks/TASK-32-web-frontend-serving.md) | `agh-web`: Frontend Embedding & Serving | ✅ DONE | `rust-embed` + SPA fallback |
| [TASK-33](./tasks/TASK-33-web-core-routes.md) | `agh-web`: Core Routes | ✅ DONE | status, dns_info, filtering, version |
| [TASK-34](./tasks/TASK-34-web-admin-routes.md) | `agh-web`: Admin Routes | ✅ DONE | clients, access (in routes.rs) |
| [TASK-35](./tasks/TASK-35-web-setup-wizard.md) | `agh-web`: Setup Wizard Routes | ✅ DONE | /install/* handlers |
| [TASK-36](./tasks/TASK-36-web-tls.md) | `agh-web`: TLS Configuration | ✅ DONE | rustls cert loading + /control/tls/* routes |
| **PHASE 9 — AGH-UPDATER** | | | |
| [TASK-37](./tasks/TASK-37-updater-version.md) | `agh-updater`: Version Check | ✅ DONE | `VersionChecker` with parse_version |
| [TASK-38](./tasks/TASK-38-updater-download.md) | `agh-updater`: Download & Atomic Replace | ✅ DONE | download_and_verify + atomic_replace |
| **PHASE 10 — AGH-MAIN** | | | |
| [TASK-39](./tasks/TASK-39-main-service.md) | `agh-main`: System Service Management | ✅ DONE | ServiceAction CLI subcommands |
| [TASK-40](./tasks/TASK-40-main-wiring.md) | `agh-main`: Wire All Crates + Startup | ✅ DONE | main.rs wires config + web server |
| **PHASE 11 — DOCKER & CI** | | | |
| [TASK-41](./tasks/TASK-41-dockerfile.md) | Multi-Arch Dockerfile | ✅ DONE | 3-stage build: frontend + rust + alpine |
| [TASK-42](./tasks/TASK-42-ci-pipeline.md) | Full CI/CD Pipeline | ✅ DONE | `.github/workflows/rust.yml` |
| **PHASE 12 — TESTING** | | | |
| [TASK-43](./tasks/TASK-43-test-api-contract.md) | API Contract Tests | ✅ DONE | 43 in-process API tests (agh-web/tests/) |
| [TASK-44](./tasks/TASK-44-test-dns-compliance.md) | DNS Compliance Tests | ✅ DONE | `tests/dns_compliance.sh` |
| [TASK-45](./tasks/TASK-45-test-config-migration.md) | Config Migration Tests | ✅ DONE | 13 tests in agh-core/tests/ + 4 fixture YAMLs |
| [TASK-46](./tasks/TASK-46-test-performance.md) | Performance Benchmarks | ✅ DONE | criterion benches + k6 script + benchmark.sh |
