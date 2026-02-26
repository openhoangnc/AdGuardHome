# TASK-18: `agh-dns` — DNS-over-HTTPS (DoH) + DNS-over-TLS (DoT)

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-15 ✅ (Core DNS server — `AdGuardRequestHandler` available for reuse)

## Objective
Add DoH support via an axum HTTP handler and DoT support via `tokio-rustls`. Both must share the same filtering + upstream logic as the plain DNS server (no code duplication — pass through the same `AdGuardRequestHandler`).

---

## Checklist

### DNS-over-HTTPS (DoH)

- [ ] Create `src/doh.rs`:
  ```rust
  /// Axum handler for GET /dns-query?dns=<base64url>
  pub async fn doh_get_handler(
      Query(params): Query<DohParams>,
      State(handler): State<Arc<AdGuardRequestHandler>>,
  ) -> impl IntoResponse;

  /// Axum handler for POST /dns-query with body = raw DNS wire format
  pub async fn doh_post_handler(
      State(handler): State<Arc<AdGuardRequestHandler>>,
      body: Bytes,
  ) -> impl IntoResponse;
  ```
- [ ] `GET /dns-query?dns=<base64url>` — decode base64url, parse DNS message, resolve, return `application/dns-message`
- [ ] `POST /dns-query` — body is raw DNS wire format (`application/dns-message`)
- [ ] Return `Content-Type: application/dns-message`
- [ ] Return HTTP 400 for malformed DNS messages
- [ ] Return HTTP 415 for wrong Content-Type (POST)
- [ ] Mount this handler on the main axum router in `agh-web` — NOT a separate server (DoH shares port 443 with the web UI or port 80 for non-TLS)

### DNS-over-TLS (DoT)

- [ ] Create `src/dot.rs`:
  ```rust
  pub struct DotServer {
      addr: SocketAddr,          // default: 0.0.0.0:853
      tls_config: Arc<ServerConfig>,
      handler: Arc<AdGuardRequestHandler>,
  }

  impl DotServer {
      pub async fn new(tls_config: Arc<ServerConfig>, handler: Arc<AdGuardRequestHandler>, port: u16) -> Self;
      pub async fn run(self) -> Result<()>;
  }
  ```
- [ ] Accept TLS connections using `tokio-rustls` `TlsAcceptor`
- [ ] After TLS handshake, read length-prefixed DNS messages (RFC 7858: 2-byte big-endian length prefix)
- [ ] Route through the same `AdGuardRequestHandler`
- [ ] Handle connection close gracefully (EOF = normal)
- [ ] TLS certificates come from `TlsConfig` (TASK-36)

---

## Tests

```rust
#[tokio::test]
async fn test_doh_get_resolves_google() {
    // Start axum test server, send encoded DNS query for google.com
    let response = client.get("/dns-query?dns=<encoded>").send().await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["content-type"], "application/dns-message");
}

#[tokio::test]
async fn test_doh_post_resolves() { ... }

#[tokio::test]
async fn test_doh_malformed_returns_400() { ... }
```

---

## Verification
```bash
cargo test -p agh-dns doh
# Manual:
curl -H 'accept: application/dns-json' 'https://localhost:443/dns-query?name=google.com&type=A'
```

---

## Output Files
- `rust-port/crates/agh-dns/src/doh.rs`
- `rust-port/crates/agh-dns/src/dot.rs`
- Update `PROGRESS.md`: TASK-18 → ✅ DONE
