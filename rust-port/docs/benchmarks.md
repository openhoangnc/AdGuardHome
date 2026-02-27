# AdGuardHome Performance Benchmarks

> Run `bash rust-port/tests/benchmark.sh` to populate this file with actual results.
> Run `cargo bench --workspace` for micro-benchmarks.

---

## Results

| Metric | Go | Rust | Notes |
|---|---|---|---|
| HTTP sequential RPS (`/control/status`) | TBD | TBD | 10s sequential curl |
| HTTP p99 latency (k6 50 VUs) | TBD | TBD | k6 30s |
| DNS query success rate (100 queries) | TBD | TBD | via dig to localhost |
| Memory (RSS, idle) | TBD | TBD | after 10s idle |
| Binary size | TBD | TBD | stripped release |

---

## Criterion Micro-benchmarks

Run with: `cargo bench --workspace`

| Benchmark | Description |
|---|---|
| `check_domain_no_match` (1k/100k/500k rules) | Filter engine miss latency vs. ruleset size |
| `check_domain_blocked` (1k/100k/500k rules) | Filter engine hit latency vs. ruleset size |
| `engine_build_time` (1k/10k/100k/500k rules) | Engine construction cost |
| `parse_easylist_sample` | Filter list parse throughput |
| DNS cache hit | Cache lookup latency (warm cache) |
| DNS cache miss | Cache lookup latency (cold cache) |
| DNS cache insert | Cache insertion throughput |

---

## Acceptance Criteria

- [ ] HTTP p99 < 50ms under 50 concurrent users
- [ ] Memory idle < 80 MB
- [ ] DNS query success rate ≥ 99/100
- [ ] Filter engine check: < 1µs per domain for 500k rules
