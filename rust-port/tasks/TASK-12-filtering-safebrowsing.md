# TASK-12: `agh-filtering` — Safe Browsing

## Status
⬜ TODO

## Phase
Phase 3 — `agh-filtering`

## Dependencies
- TASK-11 ✅ (Rule Matcher — engine architecture established)

## Objective
Implement AdGuard's safe browsing hash-prefix DNS lookup. When safe browsing is enabled, a SHA256 hash prefix of the queried domain is sent to `sb.dns.adguard.com` via DNS TXT lookup. Port from `internal/filtering/safebrowsing.go`.

---

## Checklist

- [ ] Create `src/safebrowsing.rs`:

  ```rust
  pub struct SafeBrowsingChecker {
      resolver: TokioAsyncResolver,  // hickory-resolver for DoH to sb.dns.adguard.com
      cache: Arc<Mutex<LruCache<String, SafeBrowsingResult>>>,  // cached results
      enabled: Arc<AtomicBool>,
  }

  pub enum SafeBrowsingResult {
      Safe,
      Malware,
      Phishing,
  }

  impl SafeBrowsingChecker {
      pub fn new(enabled: bool) -> Self;
      
      /// Check if domain is flagged by safe browsing (uses hash-prefix protocol)
      pub async fn check(&self, domain: &str) -> SafeBrowsingResult;
      
      pub fn enable(&self);
      pub fn disable(&self);
  }
  ```

- [ ] Hash-prefix algorithm (matches Go's `hashPrefix` func):
  1. Lowercase the domain
  2. Compute `SHA256(domain)`
  3. Take first 4 bytes as hex prefix: e.g., `a1b2c3d4`
  4. Query TXT record: `<prefix>.sb.dns.adguard.com`
  5. If TXT record contains the full SHA256, domain is malicious
- [ ] Add LRU cache (max 10,000 entries) with TTL (default: 5 minutes)
- [ ] Thread-safe: `SafeBrowsingChecker` must be `Send + Sync`
- [ ] Handle network timeout gracefully — fail open (return `Safe` on error)

---

## Tests

```rust
#[tokio::test]
async fn test_disabled_always_safe() {
    let checker = SafeBrowsingChecker::new(false);
    let result = checker.check("malware.example.com").await;
    assert!(matches!(result, SafeBrowsingResult::Safe));
}

#[tokio::test]
async fn test_cache_hit() {
    // Verify second call uses cache (mock the DNS resolver)
}

// Integration test (requires network, mark #[ignore]):
#[tokio::test]
#[ignore]
async fn test_known_malware_domain() {
    let checker = SafeBrowsingChecker::new(true);
    // AdGuard provides a test domain for this purpose
    let result = checker.check("malware.example.test").await;
    // Only check that it doesn't panic — actual result depends on network
}
```

---

## Verification
```bash
cargo test -p agh-filtering safebrowsing
```

---

## Output Files
- `rust-port/crates/agh-filtering/src/safebrowsing.rs`
- Update `PROGRESS.md`: TASK-12 → ✅ DONE
