# TASK-13: `agh-filtering` — Safe Search Rewrites

## Status
⬜ TODO

## Phase
Phase 3 — `agh-filtering`

## Dependencies
- TASK-11 ✅ (Rule Matcher)

## Objective
Implement safe search rewriting that redirects search engine queries to their "safe" counterparts. Port from `internal/filtering/safesearch.go`. When a DNS query matches a known search engine domain, return the IP of the engine's safe-search endpoint instead.

---

## Checklist

- [ ] Create `src/safesearch.rs`:

  ```rust
  pub struct SafeSearchRewriter {
      enabled: Arc<AtomicBool>,
      custom_ips: HashMap<String, IpAddr>,  // admin overrides from config
      // Built-in mappings (compile-time) from Go's safe_search.go
      builtin: &'static [(/* domain_suffix */, /* safe_ip_v4 */, /* safe_ip_v6 */)]
  }

  impl SafeSearchRewriter {
      pub fn new(enabled: bool) -> Self;
      pub fn check(&self, domain: &str) -> Option<IpAddr>;
      pub fn enable(&self);
      pub fn disable(&self);
  }
  ```

- [ ] Built-in safe search mappings (extract exact IPs from Go's `internal/filtering/safesearch.go`):
  | Domain Pattern | Safe IPv4 | Safe IPv6 |
  |---|---|---|
  | `google.com`, `www.google.*` | `forcesafesearch.google.com` resolved | — |
  | `bing.com`, `www.bing.com` | `strict.bing.com` resolved | — |
  | `duckduckgo.com` | `safe.duckduckgo.com` resolved | — |
  | `youtube.com` | `restrict.youtube.com` resolved | — |
  | `yandex.com/ru/...` | `213.180.193.56` (Yandex safe) | — |
  
  > **Note**: Read the actual IPs from `internal/filtering/safesearch.go` — do not guess.

- [ ] For `google.com` variants, return the A/AAAA records for `forcesafesearch.google.com` (resolve once at startup, cache)
- [ ] Safe search is a **DNS rewrite**: return a synthetic A/AAAA record with the safe IP, not NXDOMAIN
- [ ] Support both IPv4 and IPv6 queries (A and AAAA)

---

## Tests

```rust
#[test]
fn test_google_rewritten() {
    let rewriter = SafeSearchRewriter::new(true);
    let ip = rewriter.check("www.google.com");
    assert!(ip.is_some());
}

#[test]
fn test_disabled_no_rewrite() {
    let rewriter = SafeSearchRewriter::new(false);
    assert!(rewriter.check("www.google.com").is_none());
}

#[test]
fn test_non_search_no_rewrite() {
    let rewriter = SafeSearchRewriter::new(true);
    assert!(rewriter.check("example.com").is_none());
}
```

---

## Verification
```bash
cargo test -p agh-filtering safesearch
```

---

## Output Files
- `rust-port/crates/agh-filtering/src/safesearch.rs`
- Update `PROGRESS.md`: TASK-13 → ✅ DONE
