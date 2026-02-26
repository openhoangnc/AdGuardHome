# TASK-10: `agh-filtering` — Blocklist Parser

## Status
⬜ TODO

## Phase
Phase 3 — `agh-filtering`

## Dependencies
- TASK-06 ✅ (`FilterConfig` type from `agh-core`)

## Objective
Implement a parser that reads AdBlock-format and `/etc/hosts`-format blocklist files into an in-memory rule set. Port logic from `internal/filtering/filtering.go` and `internal/filtering/filter.go`.

---

## Checklist

- [ ] Create `src/parser.rs`:

  ```rust
  /// A single parsed filtering rule
  pub enum FilterRule {
      /// `||example.com^` — AdBlock domain block
      DomainBlock { domain: String },
      /// `@@||example.com^` — AdBlock domain allow (whitelist)
      DomainAllow { domain: String },
      /// `*.example.com` — wildcard subdomain block
      WildcardBlock { pattern: String },
      /// `/regex/` — regex rule
      Regex { pattern: Regex },
      /// Hosts-format: `0.0.0.0 example.com`
      HostsEntry { ip: IpAddr, domain: String },
      /// `$rewrite=1.2.3.4,domain=example.com`
      Rewrite { domain: String, ip: IpAddr },
      /// Comment or empty line — skip
      Comment,
  }
  ```

- [ ] `parse_line(line: &str) -> Option<FilterRule>` — parse a single rule line
- [ ] `parse_filter(content: &str) -> Vec<FilterRule>` — parse a full filter file
- [ ] Handle AdBlock rule modifiers:
  - `||domain^` (domain anchor)
  - `@@||domain^` (exception/allowlist)
  - `|http://domain|` (exact URL — treat as domain only for DNS)
  - `! comment` and `# comment`
  - `$important` modifier (boosted priority)
  - `$dnsrewrite=...` modifier (DNS rewrite)
- [ ] Handle `/etc/hosts` format: `<ip> <hostname> [aliases...]`
- [ ] Skip obviously invalid lines without panicking
- [ ] Report parse statistics: total lines, valid rules, invalid lines

---

## Tests

```rust
#[test]
fn test_adblock_domain() {
    let rule = parse_line("||example.com^").unwrap();
    assert!(matches!(rule, FilterRule::DomainBlock { domain } if domain == "example.com"));
}

#[test]
fn test_adblock_exception() {
    let rule = parse_line("@@||safe.example.com^").unwrap();
    assert!(matches!(rule, FilterRule::DomainAllow { .. }));
}

#[test]
fn test_hosts_entry() {
    let rule = parse_line("0.0.0.0 ads.example.com").unwrap();
    assert!(matches!(rule, FilterRule::HostsEntry { .. }));
}

#[test]
fn test_comment_skipped() {
    assert!(parse_line("! This is a comment").is_none());
}

#[test]
fn test_real_easylist_sample() {
    let content = include_str!("../tests/fixtures/easylist_sample.txt");
    let rules = parse_filter(content);
    assert!(rules.len() > 0);
}
```

- [ ] Add test fixture `rust-port/crates/agh-filtering/tests/fixtures/easylist_sample.txt` with 50 sample EasyList rules

---

## Verification
```bash
cargo test -p agh-filtering parser
```

---

## Output Files
- `rust-port/crates/agh-filtering/src/parser.rs`
- `rust-port/crates/agh-filtering/tests/fixtures/easylist_sample.txt`
- Update `PROGRESS.md`: TASK-10 → ✅ DONE
