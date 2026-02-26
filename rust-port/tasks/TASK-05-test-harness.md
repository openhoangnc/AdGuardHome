# TASK-05: Integration Test Harness Setup

## Status
⬜ TODO

## Phase
Phase 1 — Preparation

## Dependencies
- TASK-03 ✅ (API audit needed to know which endpoints to test)

## Objective
Set up the test infrastructure that will be used throughout the project to verify API compatibility. Create a mock HTTP server skeleton and wire the existing Playwright E2E tests to optionally point at the Rust backend.

---

## Checklist

- [ ] Create `rust-port/tests/` directory for integration tests
- [ ] Create `rust-port/tests/api_compat/` with:
  - [ ] `mod.rs` — shared test helpers:
    - `spawn_test_server() -> TestServer` — starts `agh-main` on a random port
    - `client() -> reqwest::Client` — pre-configures auth cookie for authenticated requests
    - `assert_schema(response: Value, schema_path: &str)` — validates JSON against OpenAPI schema
  - [ ] `auth_test.rs` — tests for `POST /control/login`, `GET /control/logout`
  - [ ] `status_test.rs` — tests for `GET /control/status`
- [ ] Create `rust-port/tests/fixtures/` with:
  - [ ] `test-config.yaml` — minimal valid `AdGuardHome.yaml` for tests (no DNS, no DHCP, just HTTP)
  - [ ] `openapi.json` — converted from `openapi/openapi.yaml` for schema validation in tests
- [ ] Add to workspace `Cargo.toml`:
  ```toml
  [workspace.dev-dependencies]
  tokio-test   = "0.4"
  reqwest      = { version = "0.12", features = ["json", "cookies"] }
  serde_json   = "1"
  ```
- [ ] Update `client/` Playwright test config to support `RUST_BACKEND_URL` env var — **DO NOT modify any `client/` source; only update `client/config.ts` or the Playwright config if it exists at root level**

  > ⚠️ RULE: If the Playwright config is inside `client/`, do NOT touch it. Instead, document the manual steps to run Playwright against the Rust backend.

- [ ] Verify:
  ```bash
  cd rust-port
  cargo test --test api_compat
  ```
  (Tests will fail with "connection refused" until TASK-40 completes — the test should be marked `#[ignore]` initially)

---

## Output Files
- `rust-port/tests/api_compat/mod.rs`
- `rust-port/tests/api_compat/auth_test.rs`
- `rust-port/tests/api_compat/status_test.rs`
- `rust-port/tests/fixtures/test-config.yaml`
- `rust-port/tests/fixtures/openapi.json`
- Update `PROGRESS.md`: TASK-05 → ✅ DONE
