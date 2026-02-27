# TASK-25: `agh-stats` — Time-Series Storage (redb)

## Status
⬜ TODO

## Phase
Phase 6 — `agh-stats`

## Dependencies
- TASK-06 ✅ (`StatisticsConfig`)

## Objective
Implement circular buffer time-series stats storage in `redb`. Port from `internal/stats/`. Stats are the data behind `GET /control/stats`.

---

## Checklist

- [ ] Create `src/storage.rs`:
  ```rust
  pub struct StatsStorage {
      db: Database,
      config: StatisticsConfig,
  }

  impl StatsStorage {
      pub fn open(path: &Path, config: StatisticsConfig) -> Result<Self>;

      pub fn record(&self, entry: StatsEntry) -> Result<()>;

      pub fn get_stats(&self, period: StatsPeriod) -> Result<StatsResponse>;
      pub fn reset(&self) -> Result<()>;
  }

  pub struct StatsEntry {
      pub timestamp:   DateTime<Utc>,
      pub client_ip:   IpAddr,
      pub domain:      String,
      pub response_ms: u32,
      pub blocked:     bool,
      pub blocked_svc: Option<String>,
  }
  ```

- [ ] `redb` table schema (one table per metric):
  - `QUERIES_HOUR` — key: unix_hour (u64), value: query_count (u64)
  - `BLOCKED_HOUR` — same shape
  - `TOP_DOMAINS` — key: domain (str), value: count (u64)
  - `TOP_CLIENTS` — key: ip_string (str), value: count (u64)
  - `TOP_BLOCKED` — key: domain (str), value: count (u64)
- [ ] Retention window: entries older than `config.interval` (in days) are pruned on every write
- [ ] `StatsResponse` matches Go's `GET /control/stats` JSON exactly:
  ```json
  {
    "num_dns_queries": 0,
    "num_blocked_filtering": 0,
    "num_replaced_safebrowsing": 0,
    "num_replaced_safesearch": 0,
    "num_replaced_parental": 0,
    "avg_processing_time": 0.0,
    "top_queried_domains": [],
    "top_clients": [],
    "top_blocked_domains": [],
    "dns_queries": [],
    "blocked_filtering": [],
    "replaced_safebrowsing": [],
    "replaced_parental": []
  }
  ```
- [ ] `dns_queries` / `blocked_filtering` arrays: one element per hour, for the last N hours (N = period)

---

## Tests

```rust
#[test]
fn test_record_and_retrieve() { ... }

#[test]
fn test_retention_prunes_old() { ... }

#[test]
fn test_reset_clears_all() { ... }
```

---

## Verification
```bash
cargo test -p agh-stats storage
```

---

## Output Files
- `rust-port/crates/agh-stats/src/storage.rs`
- Update `PROGRESS.md`: TASK-25 → ✅ DONE
