# TASK-24: `agh-dhcp` — HTTP API Handlers

## Status
⬜ TODO

## Phase
Phase 5 — `agh-dhcp`

## Dependencies
- TASK-23 ✅ (LeaseStore)
- TASK-03 ✅ (API audit — exact DHCP endpoint schemas)

## Objective
Implement all DHCP HTTP API handlers as axum route functions. These are registered by `agh-web` (TASK-33). The handler functions live in `agh-dhcp` to keep ownership close to the DHCP logic.

---

## Checklist

Based on `openapi/openapi.yaml` DHCP section, implement:

- [ ] `GET /control/dhcp/status`:
  ```json
  {
    "enabled": true,
    "interface_name": "eth0",
    "v4": { "gateway_ip": "...", "subnet_mask": "...", "range_start": "...", "range_end": "...", "lease_duration": 86400 },
    "v6": { ... },
    "leases": [...],
    "static_leases": [...]
  }
  ```
- [ ] `POST /control/dhcp/set_config` — update DHCP config and restart server
- [ ] `GET /control/dhcp/interfaces` — list network interfaces with IP/MAC info
- [ ] `POST /control/dhcp/add_static_lease` — `{ mac, ip, hostname }`
- [ ] `POST /control/dhcp/remove_static_lease` — `{ mac, ip, hostname }`
- [ ] `POST /control/dhcp/update_static_lease` — `{ mac, ip, hostname }` (for update)
- [ ] `POST /control/dhcp/reset` — delete all leases and reset
- [ ] `GET /control/dhcp/find_active_dhcp` — scan network to check for existing DHCP servers (send DHCP DISCOVER, wait 200ms)

For each handler:
- [ ] Match exact JSON field names from `openapi/openapi.yaml`
- [ ] Return correct HTTP status codes (200 OK, 400 Bad Request for invalid IP, 500 for server errors)
- [ ] All handlers guarded by auth middleware (TASK-31)

---

## Tests

```rust
#[tokio::test]
async fn test_status_response_shape() {
    // POST to test server, check JSON structure matches spec
}

#[tokio::test]
async fn test_add_static_lease_ok() { ... }

#[tokio::test]
async fn test_add_static_lease_duplicate_rejected() { ... }
```

---

## Verification
```bash
cargo test -p agh-dhcp api
```

---

## Output Files
- `rust-port/crates/agh-dhcp/src/http.rs`
- Update `PROGRESS.md`: TASK-24 → ✅ DONE
