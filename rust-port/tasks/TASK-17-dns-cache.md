# TASK-17: `agh-dns` — DNS Cache

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-15 ✅ (DNS server core)

## Objective
Implement a TTL-respecting DNS response cache. This reduces upstream load and latency for repeated queries. Port from `internal/dnsforward/cache.go`.

---

## Checklist

- [ ] Create `src/cache.rs`:

  ```rust
  pub struct DnsCache {
      entries: Arc<Mutex<LruCache<CacheKey, CacheEntry>>>,
      config: CacheConfig,
  }

  #[derive(Hash, Eq, PartialEq)]
  struct CacheKey {
      name:  String,
      qtype: RecordType,
      class: DNSClass,
  }

  struct CacheEntry {
      response:      DnsResponse,
      inserted_at:   Instant,
      original_ttl:  u32,
  }

  pub struct CacheConfig {
      pub enabled:   bool,
      pub size:      usize,    // max entries, default 1000
      pub min_ttl:   u32,      // override minimum TTL (0 = disabled)
      pub max_ttl:   u32,      // cap TTL at this value (0 = disabled)
      pub optimistic: bool,    // serve stale while refreshing in background
  }

  impl DnsCache {
      pub fn new(config: CacheConfig) -> Self;

      /// Look up a cached response, decrementing TTL by elapsed time
      pub fn get(&self, key: &CacheKey) -> Option<DnsResponse>;

      /// Store a response (only if it has at least one record with TTL > 0)
      pub fn put(&self, key: CacheKey, response: DnsResponse);

      pub fn clear(&self);
      pub fn len(&self) -> usize;
  }
  ```

- [ ] TTL decrement: when serving from cache, reduce each record's TTL by the seconds elapsed since insertion
- [ ] If TTL hits 0 while serving: remove from cache and return a miss
- [ ] Negative caching: cache NXDOMAIN responses with the SOA TTL per RFC 2308
- [ ] Do NOT cache:
  - Responses with SERVFAIL
  - Responses with TTL=0 (unless `min_ttl > 0`)
  - Queries from clients with per-client `use_global_cache = false` (future)
- [ ] Optimistic caching mode: serve stale entry immediately, trigger async refresh in background

---

## Tests

```rust
#[test]
fn test_cache_hit_decrements_ttl() {
    let cache = DnsCache::new(CacheConfig::default());
    let response = make_response("google.com", 60); // TTL = 60s
    cache.put(key("google.com", A), response);
    // fast-forward 10s
    let hit = cache.get(&key("google.com", A)).unwrap();
    assert_eq!(hit.answers[0].ttl(), 50); // 60 - 10 = 50
}

#[test]
fn test_expired_entry_returns_miss() { ... }

#[test]
fn test_nxdomain_cached() { ... }

#[test]
fn test_servfail_not_cached() { ... }
```

---

## Verification
```bash
cargo test -p agh-dns cache
```

---

## Output Files
- `rust-port/crates/agh-dns/src/cache.rs`
- Update `PROGRESS.md`: TASK-17 → ✅ DONE
