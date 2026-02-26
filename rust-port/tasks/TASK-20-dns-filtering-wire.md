# TASK-20: `agh-dns` — Wire Filtering Engine into DNS

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-15 ✅ (Core DNS server)
- TASK-11 ✅ (FilteringEngine)
- TASK-12 ✅ (Safe Browsing)
- TASK-13 ✅ (Safe Search)

## Objective
Connect the full filtering pipeline into the DNS request handler. This includes per-client settings, blocklist matching, safe browsing, parental controls, and safe search rewrites — and reporting each match to querylog and stats.

---

## Checklist

- [ ] Expand `AdGuardRequestHandler` with full filtering pipeline:

  ```
  For each DNS query:
    1. client = ClientRegistry.find(source_ip)
    2. settings = client?.settings ?? GlobalConfig
    3. if settings.filtering_enabled:
         result = FilteringEngine.check_domain(fqdn)
         if Blocked → respond 0.0.0.0 / NXDOMAIN; log to querylog; increment stats.blocked
         if Allowed → continue
         if Rewrite → respond with rewrite IP; log
    4. if settings.safesearch_enabled:
         rewrite_ip = SafeSearchRewriter.check(fqdn)
         if Some(ip) → respond with ip; log
    5. if settings.safebrowsing_enabled (async):
         result = SafeBrowsingChecker.check(fqdn).await
         if Malware/Phishing → return block page IP; log
    6. if settings.parental_enabled (async):
         result = ParentalChecker.check(fqdn).await
         if Blocked → return block page IP; log
    7. forward to upstream; cache; return
    8. log to querylog (always, unless client.ignore_querylog)
    9. update stats (always, unless client.ignore_statistics)
  ```

- [ ] Implement both global and per-client filtering settings lookup
- [ ] `ParentalChecker` — same hash-prefix protocol as SafeBrowsing but using `pc.dns.adguard.com` (can reuse SafeBrowsing code with different endpoint)
- [ ] Blocked services list: each service maps to a set of domains; check query against blocked service domains
- [ ] Stats channel: `tokio::sync::mpsc::channel(10000)` to decouple stats writes from the DNS hot path
- [ ] Querylog channel: same pattern, non-blocking send (drop if channel full)

---

## Tests

```rust
#[tokio::test]
async fn test_blocked_domain_logged_in_querylog() { ... }

#[tokio::test]
async fn test_per_client_override_allows_globally_blocked() { ... }

#[tokio::test]
async fn test_safesearch_rewrite_returned() { ... }

#[tokio::test]
async fn test_stats_incremented_on_block() { ... }
```

---

## Verification
```bash
cargo test -p agh-dns filtering_wire
```

---

## Output Files
- Updated `rust-port/crates/agh-dns/src/server.rs`
- `rust-port/crates/agh-dns/src/parental.rs`
- Update `PROGRESS.md`: TASK-20 → ✅ DONE
