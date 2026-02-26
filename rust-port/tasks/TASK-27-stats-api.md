# TASK-27: `agh-stats` — HTTP API Handlers

## Status
⬜ TODO

## Phase
Phase 6 — `agh-stats`

## Dependencies
- TASK-26 ✅ (StatsService)
- TASK-03 ✅ (API audit for stats endpoints)

## Objective
Implement stats HTTP API handlers.

---

## Checklist

- [ ] Create `src/http.rs`:

- [ ] `GET /control/stats` — return `StatsResponse` JSON
- [ ] `GET /control/stats_config` — return current stats config:
  ```json
  { "enabled": true, "interval": 1, "ignored": [] }
  ```
- [ ] `POST /control/stats_config` — update config (enable/disable, set interval, ignored clients):
  - Validate: `interval` must be 1|7|30|90
  - If disabled: stop recording, still return empty stats
- [ ] `POST /control/stats_reset` — clear all data (calls `StatsStorage::reset()`)

All stats handlers must:
- Return `200 OK` with `Content-Type: application/json`
- Be gated behind auth middleware

---

## Tests

```rust
#[tokio::test]
async fn test_stats_get_returns_json_with_correct_fields() { ... }

#[tokio::test]
async fn test_stats_config_update() { ... }

#[tokio::test]
async fn test_stats_reset() { ... }
```

---

## Verification
```bash
cargo test -p agh-stats http
```

---

## Output Files
- `rust-port/crates/agh-stats/src/http.rs`
- Update `PROGRESS.md`: TASK-27 → ✅ DONE
