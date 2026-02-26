# TASK-34: `agh-web` — Admin Routes (clients, access, profile)

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-33 ✅ (Router exists)
- TASK-09 ✅ (ClientRegistry)

## Objective
Implement client management, access control, user profile, and version check API routes.

---

## Checklist

### Client Management (`src/routes/clients.rs`)

- [ ] `GET /control/clients` — list all persistent + runtime clients
- [ ] `POST /control/clients/add` — add persistent client
- [ ] `POST /control/clients/delete` — remove persistent client by name
- [ ] `POST /control/clients/update` — update persistent client
- [ ] `GET /control/clients/search` — `?ip=1.2.3.4` — resolve IP to client info (name, MAC, etc.)

### Access Control (`src/routes/access.rs`)

- [ ] `GET /control/access/list` — list allowed clients, disallowed clients, blocked hosts:
  ```json
  { "allowed_clients": [], "disallowed_clients": [], "blocked_hosts": [] }
  ```
- [ ] `POST /control/access/set` — update access control lists

### User Profile & i18n (`src/routes/profile.rs`)

- [ ] `GET /control/profile` — return `{ "name": "admin", "language": "en", "theme": "auto" }` (from current session user)
- [ ] `POST /control/profile/update` — update user profile (name, password, language, theme)
- [ ] `POST /control/change_language` — shortcut to change UI language

### Version Check (`src/routes/version.rs`)

- [ ] `GET /control/version.json` — return:
  ```json
  {
    "new_version": null,
    "announcement": "",
    "announcement_url": "",
    "can_autoupdate": false
  }
  ```
  If update checking is enabled, proxy to `https://static.adguard.com/adguardhome/{channel}/version.json`
- [ ] `POST /control/update` — trigger self-update via `agh-updater`

---

## Tests

```rust
#[tokio::test]
async fn test_add_client_ok() { ... }
#[tokio::test]
async fn test_add_duplicate_client_rejected() { ... }
#[tokio::test]
async fn test_profile_returns_username() { ... }
```

---

## Verification
```bash
cargo test -p agh-web admin_routes
```

---

## Output Files
- `rust-port/crates/agh-web/src/routes/clients.rs`
- `rust-port/crates/agh-web/src/routes/access.rs`
- `rust-port/crates/agh-web/src/routes/profile.rs`
- `rust-port/crates/agh-web/src/routes/version.rs`
- Update `PROGRESS.md`: TASK-34 → ✅ DONE
