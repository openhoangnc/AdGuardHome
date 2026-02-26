# TASK-30: `agh-querylog` — HTTP API Handlers

## Status
⬜ TODO

## Phase
Phase 7 — `agh-querylog`

## Dependencies
- TASK-29 ✅ (QueryLogService)

## Objective
Implement all querylog HTTP API handlers.

---

## Checklist

- [ ] Create `src/http.rs`:

- [ ] `GET /control/querylog` — paginated query log:
  - Query params: `older_than` (ISO8601), `limit` (int), `search` (string), `response_status`, `client`
  - Response: `{ "data": [...], "oldest": "<ISO8601>" }`
  - Each entry must match the Go response fields exactly (from `openapi.yaml`)
- [ ] `GET /control/querylog_info` — returns config:
  ```json
  { "enabled": true, "interval": 90, "anonymize_client_ip": false, "ignored": [] }
  ```
- [ ] `POST /control/querylog_config` — update config:
  - Fields: `enabled`, `interval` (days: 1|7|30|90), `anonymize_client_ip`, `ignored` (list of client IDs)
- [ ] `POST /control/querylog_clear` — wipe current and rotated files

---

## Tests

```rust
#[tokio::test]
async fn test_querylog_get_returns_data() { ... }

#[tokio::test]
async fn test_querylog_pagination_cursor() { ... }

#[tokio::test]
async fn test_querylog_config_update() { ... }
```

---

## Verification
```bash
cargo test -p agh-querylog http
```

---

## Output Files
- `rust-port/crates/agh-querylog/src/http.rs`
- Update `PROGRESS.md`: TASK-30 → ✅ DONE
