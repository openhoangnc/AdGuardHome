# TASK-35: `agh-web` — Setup Wizard Routes

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-31 ✅ (Auth — firstRun detection)
- TASK-33 ✅ (Router base)

## Objective
Implement the installation/setup wizard API. These routes are accessible **without authentication** during first run. Port from `internal/home/install.go`.

---

## Checklist

- [ ] Create `src/routes/install.rs`:

  All routes under `/control/install/` are **public** (no auth required):

- [ ] `GET /control/install/get_addresses` — return available interfaces and their IPs:
  ```json
  {
    "web": { "ip": ["0.0.0.0", "127.0.0.1"], "port": 3000, "status": "" },
    "dns": { "ip": ["0.0.0.0"], "port": 53, "status": "" },
    "interfaces": {
      "eth0": { "name": "eth0", "mtu": 1500, "hardware_address": "...", "flags": ["up", "broadcast"], "ip_addresses": ["192.168.1.2/24"] }
    }
  }
  ```
- [ ] `POST /control/install/check_config` — validate the proposed config (port availability, interface existence):
  Request: `{ "web": { "ip": "0.0.0.0", "port": 3000 }, "dns": { "ip": "0.0.0.0", "port": 53 }, "set_static_ip": false }`
  - Check web port not in use (bind test)
  - Check DNS port not in use
  - Return `{ "web": { "status": "" }, "dns": { "status": "" }, "static_ip": { "static": false, "ip": "..." } }`
- [ ] `POST /control/install/configure` — finalize setup:
  - Write initial `AdGuardHome.yaml` with web username/password, DNS/HTTP ports
  - Hash password with bcrypt
  - Save config atomically
  - Set `firstRun = false`
- [ ] Guard all other `/control/` routes: if `firstRun = true` AND the route is NOT `/control/install/*`, redirect to `/install.html` (or return 403 with `{ "error": "setup not complete" }`)

---

## Tests

```rust
#[tokio::test]
async fn test_get_addresses_lists_interfaces() { ... }

#[tokio::test]
async fn test_configure_creates_config() {
    // POST with valid credentials, verify config file exists and user is created
}

#[tokio::test]
async fn test_protected_routes_blocked_during_first_run() { ... }
```

---

## Verification
```bash
cargo test -p agh-web setup_wizard
```

---

## Output Files
- `rust-port/crates/agh-web/src/routes/install.rs`
- Update `PROGRESS.md`: TASK-35 → ✅ DONE
