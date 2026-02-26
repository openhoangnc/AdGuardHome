# TASK-31: `agh-web` — Auth & Session Management

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-06 ✅ (`User` struct, `HttpConfig`)

## Objective
Implement session-based authentication using `redb` as the session store. Replicate the exact cookie name, rate limiting, and trusted proxy behavior of the Go `agh_session` auth system. Port from `internal/home/auth.go`.

---

## Checklist

- [ ] Create `src/auth.rs`:
  ```rust
  pub struct AuthManager {
      sessions: Database,     // redb sessions.db
      users:    Arc<RwLock<Vec<User>>>,
      limiter:  RateLimiter,  // per-IP failed login count
      trusted_proxies: Vec<IpNetwork>,
  }

  impl AuthManager {
      pub fn new(db_path: &Path, users: Vec<User>, trusted_proxies: Vec<String>) -> Result<Self>;

      /// Validate credentials, create session, return session token
      pub fn login(&self, username: &str, password: &str, client_ip: IpAddr) -> Result<SessionToken, AuthError>;
      
      /// Validate session token, return user info
      pub fn validate_session(&self, token: &str) -> Option<User>;
      
      pub fn logout(&self, token: &str);
  }
  ```

- [ ] Session storage in `redb`:
  - Table: `sessions` — key: token (str, 64 hex chars), value: `SessionData` (user, created_at, last_seen)
  - Sessions expire after 30 days of inactivity
- [ ] Cookie: `Set-Cookie: agh_session=<token>; Path=/; HttpOnly; SameSite=Strict`
  - If HTTPS: additionally `Secure`
- [ ] Password verification: Go uses `bcrypt` — use `bcrypt` crate (exact same format)
- [ ] Rate limiter: track failed logins per IP, block for N minutes after M failures (match Go's `authRateLimiter` defaults: 5 failures → 15min block)
- [ ] Trusted proxies: if `X-Forwarded-For` is from a trusted proxy IP, use the forwarded IP for rate limiting

- [ ] Implement axum middleware `auth_required`:
  ```rust
  pub async fn auth_required(State(auth): State<Arc<AuthManager>>, req: Request, next: Next) -> Response {
      // extract agh_session cookie → validate → attach user to request extensions
      // if invalid → return 401 or redirect to /
  }
  ```

- [ ] First-run bypass: if no users configured (`firstRun = true`), all `/control/` requests are allowed (setup wizard)

---

## Tests

```rust
#[test]
fn test_login_success_returns_session() { ... }

#[test]
fn test_wrong_password_rejected() { ... }

#[test]
fn test_rate_limiter_blocks_after_5_failures() { ... }

#[test]
fn test_session_validates_correctly() { ... }

#[test]
fn test_expired_session_rejected() { ... }
```

---

## Verification
```bash
cargo test -p agh-web auth
```

---

## Output Files
- `rust-port/crates/agh-web/src/auth.rs`
- Update `PROGRESS.md`: TASK-31 → ✅ DONE
