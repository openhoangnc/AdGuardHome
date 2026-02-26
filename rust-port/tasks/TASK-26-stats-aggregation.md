# TASK-26: `agh-stats` — Stats Aggregation

## Status
⬜ TODO

## Phase
Phase 6 — `agh-stats`

## Dependencies
- TASK-25 ✅ (StatsStorage)

## Objective
Implement the multi-period aggregation logic for stats: compute totals across configurable windows (24h, 7d, 30d, 90d). Build the `StatsService` that is the public interface for the DNS server to record stats and for the API to read them.

---

## Checklist

- [ ] Create `src/aggregation.rs`:
  ```rust
  pub struct StatsService {
      storage: Arc<StatsStorage>,
      tx:      mpsc::Sender<StatsEntry>,  // async write channel
  }

  impl StatsService {
      pub fn new(storage: Arc<StatsStorage>) -> (Self, mpsc::Receiver<StatsEntry>);

      /// Non-blocking record — drops if channel full (DNS hot path)
      pub fn record(&self, entry: StatsEntry);

      /// Aggregated stats for the configured retention period
      pub async fn get_stats(&self) -> Result<StatsResponse>;

      /// Specific period
      pub async fn get_stats_period(&self, period: StatsPeriod) -> Result<StatsResponse>;
  }
  ```
- [ ] Background writer task: drain the mpsc channel and write to `StatsStorage` in batches (every 1 second)
- [ ] Aggregation logic for `dns_queries` / `blocked_filtering` arrays:
  - 24h period: 24 hourly buckets
  - 7d period: 7 daily buckets (aggregate hourly → daily)
  - 30d period: 30 daily buckets
  - Default: use `config.interval` (in days)
- [ ] Average processing time: running mean across all queries in the period

---

## Tests

```rust
#[tokio::test]
async fn test_record_increments_count() { ... }

#[tokio::test]
async fn test_24h_has_24_buckets() {
    let stats = service.get_stats_period(StatsPeriod::Hours24).await.unwrap();
    assert_eq!(stats.dns_queries.len(), 24);
}
```

---

## Verification
```bash
cargo test -p agh-stats aggregation
```

---

## Output Files
- `rust-port/crates/agh-stats/src/aggregation.rs`
- Update `PROGRESS.md`: TASK-26 → ✅ DONE
