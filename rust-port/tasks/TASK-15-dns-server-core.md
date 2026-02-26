# TASK-15: `agh-dns` — Core DNS Server (UDP/TCP port 53)

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-11 ✅ (`FilteringEngine` ready to use)
- TASK-09 ✅ (Client Registry for per-client lookups)

## Objective
Build the core DNS server on `hickory-server` listening on UDP port 53 (primary) and TCP port 53 (for large responses). Every DNS query goes through the filtering engine. This is the single most important performance-critical component.

---

## Checklist

- [ ] Create `src/server.rs`:

  ```rust
  pub struct DnsServer {
      catalog: Catalog,  // hickory-server Catalog
      filtering: Arc<RwLock<Arc<FilteringEngine>>>,
      clients: Arc<ClientRegistry>,
      upstream: Arc<UpstreamResolver>,  // from TASK-16
      cache: Arc<DnsCache>,             // from TASK-17
      config: Arc<RwLock<DnsConfig>>,
  }

  impl DnsServer {
      pub async fn new(config: DnsConfig, filtering: Arc<..>, clients: Arc<..>) -> Result<Self>;
      pub async fn run(self) -> Result<()>;
  }
  ```

- [ ] Bind to `0.0.0.0:53` (configurable via `DnsConfig.bind_host` / `bind_port`)
- [ ] Implement custom `RequestHandler` trait for `hickory-server`:
  ```rust
  struct AdGuardRequestHandler { ... }
  #[async_trait]
  impl RequestHandler for AdGuardRequestHandler {
      async fn handle_request(&self, request: &Request, response_handle: ResponseHandle) -> ResponseInfo;
  }
  ```
  Handler logic per query:
  1. Extract FQDN from request, strip trailing `.`
  2. Look up client in `ClientRegistry` by source IP → get per-client settings
  3. Check `FilteringEngine.check_domain(fqdn)`:
     - `Blocked` → return `0.0.0.0` (A) / `::` (AAAA) synthetic response OR `NXDOMAIN` per config
     - `Rewrite` → return the rewrite IP
     - `NoMatch` → continue
  4. Check cache (`DnsCache`) → cache hit → return cached response
  5. Forward to upstream resolver
  6. Store in cache
  7. Return response
  8. Log to querylog (async, non-blocking via channel)
  9. Update stats (async, non-blocking via channel)

- [ ] Use `SO_REUSEPORT` via `socket2` for zero-downtime restarts (Linux)
- [ ] Handle `EDNS0` (RFC 6891) — pass through `OPT` records
- [ ] Return minimal NXDOMAIN response when domain is blocked (configurable: use `0.0.0.0` or NXDOMAIN)
- [ ] Handle PTR queries for blocked domains correctly

---

## Tests

```rust
#[tokio::test]
async fn test_blocked_domain_returns_nxdomain() { ... }

#[tokio::test]
async fn test_allowed_domain_resolves() { ... }

#[tokio::test]
async fn test_per_client_rule_applies() { ... }

#[tokio::test]
async fn test_ptr_query_passthrough() { ... }
```
Use a loopback DNS client sending raw UDP packets to test port.

---

## Verification
```bash
cargo test -p agh-dns server
# Manual test:
dig @127.0.0.1 -p 5353 google.com
dig @127.0.0.1 -p 5353 ads.example.com   # should return 0.0.0.0
```

---

## Output Files
- `rust-port/crates/agh-dns/src/server.rs`
- Update `PROGRESS.md`: TASK-15 → ✅ DONE
