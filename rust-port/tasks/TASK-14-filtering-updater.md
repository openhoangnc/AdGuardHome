# TASK-14: `agh-filtering` — Blocklist Auto-Updater

## Status
⬜ TODO

## Phase
Phase 3 — `agh-filtering`

## Dependencies
- TASK-11 ✅ (Rule Matcher — knows how to reload rules)

## Objective
Implement the blocklist download and auto-update scheduler. Periodically downloads filter lists from their URLs, parses them, and hot-reloads the `FilteringEngine` without downtime. Also watch `/etc/hosts` for changes.

---

## Checklist

- [ ] Create `src/updater.rs`:

  ```rust
  pub struct FilterUpdater {
      engine: Arc<RwLock<Arc<FilteringEngine>>>,
      config: Arc<RwLock<Vec<FilterConfig>>>,
      http: reqwest::Client,
      cache_dir: PathBuf,
  }

  impl FilterUpdater {
      pub fn new(engine: Arc<RwLock<Arc<FilteringEngine>>>, cache_dir: PathBuf) -> Self;
      
      /// Download and reload a single filter list
      pub async fn update_filter(&self, filter: &FilterConfig) -> Result<UpdateStats, UpdaterError>;
      
      /// Update all enabled filters
      pub async fn update_all(&self) -> Vec<Result<UpdateStats, UpdaterError>>;
      
      /// Start the background auto-update scheduler
      pub fn start_scheduler(self: Arc<Self>, interval: Duration) -> tokio::task::JoinHandle<()>;
  }

  pub struct UpdateStats {
      pub filter_id: u64,
      pub rules_count: usize,
      pub updated_at: DateTime<Utc>,
  }
  ```

- [ ] Download logic:
  1. HTTP GET the filter URL (with `reqwest`, follow redirects, max 50 MB)
  2. Check `Last-Modified` / `ETag` headers — skip re-parsing if unchanged
  3. Save raw content to `cache_dir/<filter_id>.txt`
  4. Parse with `parse_filter()` from TASK-10
  5. If parse succeeds, atomically replace the engine
- [ ] `/etc/hosts` watcher using `notify` crate:
  ```rust
  pub async fn watch_hosts(engine: Arc<RwLock<Arc<FilteringEngine>>>) -> notify::RecommendedWatcher;
  ```
  On file change event: re-read `/etc/hosts`, merge with existing blocklist rules, reload engine.
- [ ] User-added rules (from `user_rules` in config) always loaded on top of downloaded lists
- [ ] Scheduler interval: configurable, default 12 hours (matching Go's default)

---

## Tests

```rust
#[tokio::test]
async fn test_update_from_file_url() {
    // Use a `file://` URL pointing to a test fixture
}

#[tokio::test]
async fn test_etag_skips_reparse() { ... }

#[tokio::test]
async fn test_engine_reloaded_after_update() { ... }
```

---

## Verification
```bash
cargo test -p agh-filtering updater
```

---

## Output Files
- `rust-port/crates/agh-filtering/src/updater.rs`
- Update `PROGRESS.md`: TASK-14 → ✅ DONE
