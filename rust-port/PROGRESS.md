# AdGuardHome Rust Port — Progress Tracker

> **Last updated**: 2026-02-26  
> **Rules**: See [RULES.md](./RULES.md) — read before starting any task  
> **Task files**: `rust-port/tasks/TASK-XX-*.md`

---

## 📊 Overall Progress

| Phase | Tasks | Done | In Progress | Blocked |
|---|---|---|---|---|
| Phase 0 — Scaffold | 3 | 0 | 0 | 0 |
| Phase 1 — Preparation | 3 | 0 | 0 | 0 |
| Phase 2 — `agh-core` | 4 | 0 | 0 | 0 |
| Phase 3 — `agh-filtering` | 5 | 0 | 0 | 0 |
| Phase 4 — `agh-dns` | 6 | 0 | 0 | 0 |
| Phase 5 — `agh-dhcp` | 4 | 0 | 0 | 0 |
| Phase 6 — `agh-stats` | 3 | 0 | 0 | 0 |
| Phase 7 — `agh-querylog` | 3 | 0 | 0 | 0 |
| Phase 8 — `agh-web` | 6 | 0 | 0 | 0 |
| Phase 9 — `agh-updater` | 2 | 0 | 0 | 0 |
| Phase 10 — `agh-main` | 2 | 0 | 0 | 0 |
| Phase 11 — Docker & CI | 2 | 0 | 0 | 0 |
| Phase 12 — Testing | 4 | 0 | 0 | 0 |
| **Total** | **47** | **0** | **0** | **0** |

---

## 📋 Task Status Table

| Task ID | Name | Status | Notes |
|---|---|---|---|
| **PHASE 0 — SCAFFOLD** | | | |
| [TASK-00](./tasks/TASK-00-workspace-scaffold.md) | Cargo Workspace Scaffold | ⬜ TODO | First task — no dependencies |
| [TASK-01](./tasks/TASK-01-crate-skeletons.md) | Create All Crate Skeletons | ⬜ TODO | Needs TASK-00 |
| [TASK-02](./tasks/TASK-02-ci-skeleton.md) | CI/CD Skeleton (GitHub Actions) | ⬜ TODO | Needs TASK-01 |
| **PHASE 1 — PREPARATION** | | | |
| [TASK-03](./tasks/TASK-03-api-audit.md) | API Contract Audit | ⬜ TODO | Can run in parallel with TASK-00 |
| [TASK-04](./tasks/TASK-04-config-schema-audit.md) | Config YAML Schema Audit | ⬜ TODO | Can run in parallel with TASK-00 |
| [TASK-05](./tasks/TASK-05-test-harness.md) | Integration Test Harness Setup | ⬜ TODO | Needs TASK-03 |
| **PHASE 2 — AGH-CORE** | | | |
| [TASK-06](./tasks/TASK-06-core-config-types.md) | `agh-core`: Config Structs & Serde | ⬜ TODO | Needs TASK-01, TASK-04 |
| [TASK-07](./tasks/TASK-07-core-config-io.md) | `agh-core`: Config Read/Write & Atomic | ⬜ TODO | Needs TASK-06 |
| [TASK-08](./tasks/TASK-08-core-cli.md) | `agh-core`: CLI Argument Parsing | ⬜ TODO | Needs TASK-06 |
| [TASK-09](./tasks/TASK-09-core-client-registry.md) | `agh-core`: Client Registry | ⬜ TODO | Needs TASK-06 |
| **PHASE 3 — AGH-FILTERING** | | | |
| [TASK-10](./tasks/TASK-10-filtering-parser.md) | `agh-filtering`: Blocklist Parser | ⬜ TODO | Needs TASK-06 |
| [TASK-11](./tasks/TASK-11-filtering-matcher.md) | `agh-filtering`: Rule Matcher | ⬜ TODO | Needs TASK-10 |
| [TASK-12](./tasks/TASK-12-filtering-safebrowsing.md) | `agh-filtering`: Safe Browsing | ⬜ TODO | Needs TASK-11 |
| [TASK-13](./tasks/TASK-13-filtering-safesearch.md) | `agh-filtering`: Safe Search Rewrites | ⬜ TODO | Needs TASK-11 |
| [TASK-14](./tasks/TASK-14-filtering-updater.md) | `agh-filtering`: Blocklist Auto-Updater | ⬜ TODO | Needs TASK-11 |
| **PHASE 4 — AGH-DNS** | | | |
| [TASK-15](./tasks/TASK-15-dns-server-core.md) | `agh-dns`: Core DNS Server (UDP/TCP) | ⬜ TODO | Needs TASK-11 |
| [TASK-16](./tasks/TASK-16-dns-upstream.md) | `agh-dns`: Upstream Resolvers | ⬜ TODO | Needs TASK-15 |
| [TASK-17](./tasks/TASK-17-dns-cache.md) | `agh-dns`: DNS Cache | ⬜ TODO | Needs TASK-15 |
| [TASK-18](./tasks/TASK-18-dns-doh-dot.md) | `agh-dns`: DoH + DoT | ⬜ TODO | Needs TASK-15 |
| [TASK-19](./tasks/TASK-19-dns-doq-dnscrypt.md) | `agh-dns`: DoQ + DNSCrypt | ⬜ TODO | Needs TASK-15 |
| [TASK-20](./tasks/TASK-20-dns-filtering-wire.md) | `agh-dns`: Wire Filtering Engine | ⬜ TODO | Needs TASK-15, TASK-11 |
| **PHASE 5 — AGH-DHCP** | | | |
| [TASK-21](./tasks/TASK-21-dhcp-v4.md) | `agh-dhcp`: DHCPv4 Server | ⬜ TODO | Needs TASK-06 |
| [TASK-22](./tasks/TASK-22-dhcp-v6.md) | `agh-dhcp`: DHCPv6 Server | ⬜ TODO | Needs TASK-21 |
| [TASK-23](./tasks/TASK-23-dhcp-leases.md) | `agh-dhcp`: Lease Persistence & ARP | ⬜ TODO | Needs TASK-21 |
| [TASK-24](./tasks/TASK-24-dhcp-api.md) | `agh-dhcp`: HTTP API Handlers | ⬜ TODO | Needs TASK-23 |
| **PHASE 6 — AGH-STATS** | | | |
| [TASK-25](./tasks/TASK-25-stats-storage.md) | `agh-stats`: Time-Series Storage (redb) | ⬜ TODO | Needs TASK-06 |
| [TASK-26](./tasks/TASK-26-stats-aggregation.md) | `agh-stats`: Stats Aggregation | ⬜ TODO | Needs TASK-25 |
| [TASK-27](./tasks/TASK-27-stats-api.md) | `agh-stats`: HTTP API Handlers | ⬜ TODO | Needs TASK-26 |
| **PHASE 7 — AGH-QUERYLOG** | | | |
| [TASK-28](./tasks/TASK-28-querylog-storage.md) | `agh-querylog`: Append-Only Storage | ⬜ TODO | Needs TASK-06 |
| [TASK-29](./tasks/TASK-29-querylog-query.md) | `agh-querylog`: Pagination & Filtering | ⬜ TODO | Needs TASK-28 |
| [TASK-30](./tasks/TASK-30-querylog-api.md) | `agh-querylog`: HTTP API Handlers | ⬜ TODO | Needs TASK-29 |
| **PHASE 8 — AGH-WEB** | | | |
| [TASK-31](./tasks/TASK-31-web-auth.md) | `agh-web`: Auth & Session Management | ⬜ TODO | Needs TASK-06 |
| [TASK-32](./tasks/TASK-32-web-frontend-serving.md) | `agh-web`: Frontend Embedding & Serving | ⬜ TODO | Needs TASK-01 |
| [TASK-33](./tasks/TASK-33-web-core-routes.md) | `agh-web`: Core Routes (status, dns_info, filtering) | ⬜ TODO | Needs TASK-31, TASK-11 |
| [TASK-34](./tasks/TASK-34-web-admin-routes.md) | `agh-web`: Admin Routes (clients, access, rewrite) | ⬜ TODO | Needs TASK-33 |
| [TASK-35](./tasks/TASK-35-web-setup-wizard.md) | `agh-web`: Setup Wizard Routes | ⬜ TODO | Needs TASK-31 |
| [TASK-36](./tasks/TASK-36-web-tls.md) | `agh-web`: TLS Configuration & HTTPS | ⬜ TODO | Needs TASK-31 |
| **PHASE 9 — AGH-UPDATER** | | | |
| [TASK-37](./tasks/TASK-37-updater-version.md) | `agh-updater`: Version Check | ⬜ TODO | Needs TASK-06 |
| [TASK-38](./tasks/TASK-38-updater-download.md) | `agh-updater`: Download & Atomic Replace | ⬜ TODO | Needs TASK-37 |
| **PHASE 10 — AGH-MAIN** | | | |
| [TASK-39](./tasks/TASK-39-main-service.md) | `agh-main`: System Service Management | ⬜ TODO | Needs TASK-08 |
| [TASK-40](./tasks/TASK-40-main-wiring.md) | `agh-main`: Wire All Crates + Startup | ⬜ TODO | All crates done |
| **PHASE 11 — DOCKER & CI** | | | |
| [TASK-41](./tasks/TASK-41-dockerfile.md) | Multi-Arch Dockerfile | ⬜ TODO | Needs TASK-40 |
| [TASK-42](./tasks/TASK-42-ci-pipeline.md) | Full CI/CD Pipeline | ⬜ TODO | Needs TASK-41 |
| **PHASE 12 — TESTING** | | | |
| [TASK-43](./tasks/TASK-43-test-api-contract.md) | API Contract Tests (all endpoints) | ⬜ TODO | Needs TASK-40 |
| [TASK-44](./tasks/TASK-44-test-dns-compliance.md) | DNS Compliance Tests | ⬜ TODO | Needs TASK-20 |
| [TASK-45](./tasks/TASK-45-test-config-migration.md) | Config Migration Tests | ⬜ TODO | Needs TASK-07 |
| [TASK-46](./tasks/TASK-46-test-performance.md) | Performance Benchmarks | ⬜ TODO | Needs TASK-40 |

---

## 🏁 Legend

| Symbol | Meaning |
|---|---|
| ⬜ TODO | Not started |
| 🟡 IN PROGRESS | Agent currently working |
| ✅ DONE | Completed and tested |
| 🔴 BLOCKED | Waiting on dependency or decision |

---

## 📝 Blockers & Notes

_None yet — project not started._
