# TASK-33: `agh-web` — Core Routes (status, DNS, filtering, safebrowsing)

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-31 ✅ (Auth middleware ready)
- TASK-11 ✅ (FilteringEngine)
- TASK-06 ✅ (ConfigManager)
- TASK-03 ✅ (API audit document)

## Objective
Implement the main axum `Router` and the core DNS/filtering API routes. This is the most critical task for API compatibility.

---

## Checklist

### Router Assembly (`src/router.rs`)

- [ ] Create the main `Router` with all routes:
  ```rust
  pub fn build_router(state: AppState) -> Router {
      Router::new()
          // Public
          .route("/control/login",  post(auth::login_handler))
          .route("/control/logout", get(auth::logout_handler))
          // Auth-required
          .nest("/control", auth_routes().layer(auth_middleware))
          // Frontend (lowest priority)
          .fallback(frontend::serve_asset)
  }
  ```

### Core Status Routes (`src/routes/status.rs`)

- [ ] `GET /control/status`:
  ```json
  {
    "dns_addresses": ["0.0.0.0"],
    "dns_port": 53,
    "http_port": 3000,
    "protection_enabled": true,
    "dhcp_ava_interfaces": null,
    "running": true,
    "version": "v0.107.0-rust",
    "language": "en"
  }
  ```
- [ ] `POST /control/restart` — graceful restart (send SIGTERM to self or reload state)
- [ ] `POST /control/shutdown` — graceful shutdown

### DNS Info Routes (`src/routes/dns.rs`)

- [ ] `GET /control/dns_info` — return current DNS config (upstreams, cache, EDNS, etc.)
- [ ] `POST /control/dns_config` — update DNS config, reload DNS server
- [ ] `POST /control/test_upstream_dns` — test upstreams: `{ "upstream_dns": ["8.8.8.8"] }` → `{ "8.8.8.8": "ok" }`

### Filtering Routes (`src/routes/filtering.rs`)

- [ ] `GET /control/filtering/status` — list all enabled/disabled filter lists + user rules + stats
- [ ] `POST /control/filtering/config` — toggle filtering enabled, set update period
- [ ] `POST /control/filtering/add_url` — add new filter list by URL
- [ ] `POST /control/filtering/remove_url` — remove filter list
- [ ] `POST /control/filtering/set_url` — update filter list (enable/disable)
- [ ] `POST /control/filtering/check_host` — `{ "name": "ads.example.com" }` → check against engine
- [ ] `POST /control/filtering/refresh` — trigger immediate update of all filter lists
- [ ] `GET /control/rewrite/list` — list DNS rewrites
- [ ] `POST /control/rewrite/add` — `{ "domain": "...", "answer": "..." }`
- [ ] `POST /control/rewrite/delete`
- [ ] `POST /control/rewrite/update`

### Safe Browsing / Parental / Safe Search Routes

- [ ] `POST /control/safebrowsing/enable`, `/disable`, `GET /control/safebrowsing/status`
- [ ] `POST /control/parental/enable`, `/disable`, `GET /control/parental/status`
- [ ] `GET /control/safesearch/status`, `POST /control/safesearch/enable`, `/disable`, `POST /control/safesearch/settings`

### Blocked Services Routes

- [ ] `GET /control/blocked_services/all` — list all available blocked services
- [ ] `GET /control/blocked_services/get` — get currently enabled
- [ ] `POST /control/blocked_services/set` — set list of enabled services

---

## Tests

```rust
#[tokio::test]
async fn test_status_returns_200() { ... }
#[tokio::test]
async fn test_filtering_status_shape() { ... }
#[tokio::test]
async fn test_check_host_blocked() { ... }
#[tokio::test]
async fn test_unauthenticated_returns_403() { ... }
```

---

## Verification
```bash
cargo test -p agh-web routes
```

---

## Output Files
- `rust-port/crates/agh-web/src/router.rs`
- `rust-port/crates/agh-web/src/routes/status.rs`
- `rust-port/crates/agh-web/src/routes/dns.rs`
- `rust-port/crates/agh-web/src/routes/filtering.rs`
- Update `PROGRESS.md`: TASK-33 → ✅ DONE
