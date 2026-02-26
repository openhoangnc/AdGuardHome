# TASK-43: API Contract Tests (All Endpoints)

## Status
⬜ TODO

## Phase
Phase 12 — Testing

## Dependencies
- TASK-40 ✅ (binary runs successfully)
- TASK-05 ✅ (test harness)
- TASK-03 ✅ (API audit with all endpoint schemas)

## Objective
Run the full API compatibility test suite against the running Rust binary and verify every endpoint matches the OpenAPI spec. All tests that were marked `#[ignore]` in TASK-05 should now pass.

---

## Checklist

- [ ] Remove `#[ignore]` from integration tests that need the running server
- [ ] For each endpoint in `openapi/openapi.yaml`, add a test in `rust-port/tests/api_compat/`:

  | Test File | Endpoints Covered |
  |---|---|
  | `auth_test.rs` | `POST /control/login`, `GET /control/logout` |
  | `status_test.rs` | `GET /control/status`, `POST /control/restart` |
  | `dns_test.rs` | `GET /control/dns_info`, `POST /control/dns_config`, `POST /control/test_upstream_dns` |
  | `filtering_test.rs` | All `/control/filtering/*` routes |
  | `querylog_test.rs` | All `/control/querylog*` routes |
  | `stats_test.rs` | All `/control/stats*` routes |
  | `clients_test.rs` | All `/control/clients*` routes |
  | `dhcp_test.rs` | All `/control/dhcp/*` routes (with DHCP disabled in test config) |
  | `tls_test.rs` | TLS validate/configure routes |
  | `setup_test.rs` | All `/control/install/*` routes (firstRun mode) |

- [ ] Each test must:
  1. Verify HTTP status code matches spec
  2. Verify `Content-Type: application/json` (where applicable)
  3. Verify response body matches OpenAPI JSON schema
  4. Verify auth cookie behavior (protected routes return 401 without cookie)

- [ ] Run existing Playwright E2E frontend tests against the Rust backend:
  ```bash
  BACKEND_URL=http://localhost:3000 npx playwright test --project chromium
  ```
  All Playwright tests must pass (they run against the prod frontend embedded in the Rust binary).

---

## Acceptance Criteria
- All 47 API contract tests pass
- All Playwright E2E frontend tests pass
- No Go binary involved — Rust binary handles everything

---

## Verification
```bash
cd rust-port
cargo test --test api_compat
```

---

## Output Files
- `rust-port/tests/api_compat/*.rs` (complete test files)
- Update `PROGRESS.md`: TASK-43 → ✅ DONE
