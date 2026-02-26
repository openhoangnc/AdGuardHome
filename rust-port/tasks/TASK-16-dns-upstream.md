# TASK-16: `agh-dns` — Upstream Resolvers

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-15 ✅ (Core DNS server structure established)

## Objective
Implement the upstream DNS resolver that forwards queries to configured upstream servers. Support plain DNS, DoH, DoT, and DoQ upstreams. Implement parallel queries and fastest-response selection (like Go's `dnsproxy` library).

---

## Checklist

- [ ] Create `src/upstream.rs`:

  ```rust
  pub struct UpstreamResolver {
      upstreams: Vec<Box<dyn Upstream>>,
      bootstrap: Vec<IpAddr>,
      parallel: bool,           // query all upstreams, return fastest
      fastest_addr: bool,       // return fastest A/AAAA record
  }

  #[async_trait]
  pub trait Upstream: Send + Sync {
      async fn resolve(&self, request: &DnsRequest) -> Result<DnsResponse, UpstreamError>;
      fn address(&self) -> &str;
  }

  // Implementations:
  pub struct PlainDnsUpstream { addr: SocketAddr }
  pub struct DohUpstream      { url: String, client: reqwest::Client }
  pub struct DotUpstream      { addr: String, tls: TlsConnector }
  pub struct DoqUpstream      { endpoint: quinn::Endpoint }
  ```

- [ ] `UpstreamResolver::resolve()` logic:
  1. If `parallel = true`: send query to all upstreams simultaneously, return first non-error response
  2. If `parallel = false`: try upstreams in order, first success wins
  3. Track per-upstream latency, prefer fastest upstream in subsequent queries
- [ ] Parse upstream URL format (matching Go's `dnsproxy` format):
  - `8.8.8.8` → plain UDP DNS
  - `8.8.8.8:53` → plain UDP DNS explicit port
  - `tcp://8.8.8.8` → plain TCP DNS
  - `https://dns.cloudflare.com/dns-query` → DoH
  - `tls://1.1.1.1` → DoT
  - `quic://dns.adguard.com` → DoQ
  - `[/domain/]upstream` → per-domain upstream (split DNS)
- [ ] Implement split-horizon DNS: `[/internal.example.com/]192.168.1.1` routes `*.internal.example.com` to that upstream only
- [ ] Bootstrap resolves DoH/DoT hostnames (before encrypted DNS is up, use plain DNS against bootstrap IPs)
- [ ] Connection pooling for DoH (reuse HTTP/2 connections)
- [ ] Timeout: 2 seconds per upstream query (configurable)

---

## Tests

```rust
#[tokio::test]
async fn test_plain_dns_resolves() {
    // Use 8.8.8.8 or a local test DNS server
}

#[tokio::test]
async fn test_parallel_fastest_wins() { ... }

#[tokio::test]
async fn test_split_dns_routing() { ... }

#[tokio::test]
async fn test_upstream_url_parsing() {
    assert_eq!(parse_upstream("8.8.8.8"), UpstreamType::PlainDns);
    assert_eq!(parse_upstream("https://dns.cloudflare.com/dns-query"), UpstreamType::Doh);
}
```

---

## Verification
```bash
cargo test -p agh-dns upstream
```

---

## Output Files
- `rust-port/crates/agh-dns/src/upstream.rs`
- Update `PROGRESS.md`: TASK-16 → ✅ DONE
