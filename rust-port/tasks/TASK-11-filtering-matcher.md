# TASK-11: `agh-filtering` — Rule Matcher

## Status
⬜ TODO

## Phase
Phase 3 — `agh-filtering`

## Dependencies
- TASK-10 ✅ (Blocklist Parser)

## Objective
Build a high-performance rule matcher using `aho-corasick` + radix trie. This is the hot path — every DNS query runs through it. Port the matching logic from `internal/filtering/filtering.go`.

---

## Checklist

- [ ] Create `src/matcher.rs`:

  ```rust
  pub struct FilteringEngine {
      /// Fast exact and substring domain matcher (aho-corasick)
      blocklist:    AhoCorasick,
      allowlist:    HashSet<String>,
      wildcards:    Vec<WildcardRule>,
      regexes:      Vec<(Regex, RuleAction)>,
      rewrites:     HashMap<String, IpAddr>,
      rule_count:   usize,
  }

  pub enum FilterResult {
      Blocked  { matched_rule: String },
      Allowed  { matched_rule: String },  // allowlist match
      Rewrite  { ip: IpAddr },
      NoMatch,
  }

  impl FilteringEngine {
      pub fn build(rules: Vec<FilterRule>) -> Self;
      pub fn check_domain(&self, domain: &str) -> FilterResult;
      pub fn rule_count(&self) -> usize;
  }
  ```

- [ ] Matching priority (same as Go — in order):
  1. Allowlist rules (`@@`) — if matches, return `Allowed`
  2. Exact domain matches (hash set lookup — O(1))
  3. Wildcard matches (`*.example.com` — check if domain ends with `.example.com`)
  4. Regex matches
  5. AhoCorasick substring matches (for blocklist patterns)
  6. Return `NoMatch`
- [ ] `WildcardRule`: precompute the suffix to match against for performance
- [ ] Add `reload(new_rules: Vec<FilterRule>) -> Arc<FilteringEngine>` for hot reload without downtime
- [ ] The `FilteringEngine` must be `Send + Sync` (required for sharing across tokio tasks)

---

## Performance Requirement
Must match ≥1M domains/second on a single core against a 500k-rule blocklist (use `cargo bench` to verify).

---

## Tests

```rust
#[test]
fn test_exact_block() {
    let engine = build_engine(vec!["||ads.example.com^"]);
    assert!(matches!(engine.check_domain("ads.example.com"), FilterResult::Blocked { .. }));
}

#[test]
fn test_subdomain_blocked_by_wildcard() {
    let engine = build_engine(vec!["||example.com^"]);
    assert!(matches!(engine.check_domain("sub.example.com"), FilterResult::Blocked { .. }));
}

#[test]
fn test_allowlist_overrides_block() {
    let engine = build_engine(vec!["||example.com^", "@@||safe.example.com^"]);
    assert!(matches!(engine.check_domain("safe.example.com"), FilterResult::Allowed { .. }));
    assert!(matches!(engine.check_domain("ads.example.com"), FilterResult::Blocked { .. }));
}

#[test]
fn test_no_match() {
    let engine = build_engine(vec!["||ads.example.com^"]);
    assert!(matches!(engine.check_domain("google.com"), FilterResult::NoMatch));
}

#[bench]
fn bench_check_1m_domains(b: &mut Bencher) { ... }
```

---

## Verification
```bash
cargo test -p agh-filtering matcher
cargo bench -p agh-filtering
```

---

## Output Files
- `rust-port/crates/agh-filtering/src/matcher.rs`
- Update `PROGRESS.md`: TASK-11 → ✅ DONE
