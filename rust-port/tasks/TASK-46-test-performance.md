# TASK-46: Performance Benchmarks

## Status
⬜ TODO

## Phase
Phase 12 — Testing

## Dependencies
- TASK-40 ✅ (complete binary)

## Objective
Measure and document performance metrics. The Rust binary should show ≥2x higher DNS query throughput vs. the Go binary due to zero-GC. Identify any bottlenecks before release.

---

## Checklist

### Micro-benchmarks (cargo bench)

- [ ] `agh-filtering` benchmarks in `benches/filtering.rs`:
  ```rust
  fn bench_check_domain_no_match(c: &mut Criterion) {
      let engine = build_engine_with_n_rules(500_000);
      c.bench_function("check_domain_no_match_500k_rules", |b| {
          b.iter(|| engine.check_domain(black_box("google.com")))
      });
  }

  fn bench_check_domain_blocked(c: &mut Criterion) { ... }
  fn bench_engine_build_time(c: &mut Criterion) { ... }
  ```

- [ ] `agh-dns` benchmarks in `benches/dns.rs`:
  - DNS query handling throughput (queries/second on loopback)
  - Cache hit vs miss latency

### Load Tests (k6)

- [ ] Create `rust-port/tests/k6/dns_load.js`:
  ```javascript
  // k6 load test for HTTP + DNS (via DoH)
  import http from 'k6/http';
  import { check, sleep } from 'k6';
  
  export const options = {
    vus: 100,
    duration: '30s',
  };
  
  export default function () {
    const resp = http.get('http://localhost:3000/control/status');
    check(resp, { 'status 200': (r) => r.status === 200 });
    sleep(0.01);
  }
  ```

- [ ] Create `rust-port/tests/benchmark.sh` that:
  1. Starts Go binary on port 3000
  2. Runs `k6 run dns_load.js` → records RPS and p99 latency
  3. Starts Rust binary on port 3001
  4. Runs `k6 run dns_load.js` (pointing to 3001)
  5. Prints comparison table

- [ ] DNS throughput benchmark using `dnsperf` or `flamethrower`:
  ```bash
  # Install dnsperf: brew install dnsperf
  dnsperf -s 127.0.0.1 -p 5353 -d rust-port/tests/fixtures/dns_queries.txt -q 10000 -c 100
  ```
  - Create `dns_queries.txt` with 1000 random query lines

- [ ] Document results in `rust-port/docs/benchmarks.md`:
  ```markdown
  | Metric | Go | Rust | Ratio |
  |---|---|---|---|
  | DNS queries/sec | TBD | TBD | TBD |
  | DNS p99 latency | TBD | TBD | TBD |
  | HTTP RPS (/control/status) | TBD | TBD | TBD |
  | Memory usage (idle) | TBD | TBD | TBD |
  | Binary size | TBD | TBD | TBD |
  ```

---

## Acceptance Criteria
- DNS query throughput (Rust) ≥ 2x the Go baseline
- HTTP API latency p99 < 5ms under 100 concurrent users
- Memory usage < 50 MB idle (vs. Go's ~30-80 MB, Rust should be lower)
- Cargo bench results saved and regression-checked in CI

---

## Verification
```bash
cd rust-port
cargo bench
bash tests/benchmark.sh
```

---

## Output Files
- `rust-port/crates/agh-filtering/benches/filtering.rs`
- `rust-port/crates/agh-dns/benches/dns.rs`
- `rust-port/tests/k6/dns_load.js`
- `rust-port/tests/benchmark.sh`
- `rust-port/tests/fixtures/dns_queries.txt`
- `rust-port/docs/benchmarks.md`
- Update `PROGRESS.md`: TASK-46 → ✅ DONE
