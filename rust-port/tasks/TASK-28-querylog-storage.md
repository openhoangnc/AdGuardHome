# TASK-28: `agh-querylog` — Append-Only Storage

## Status
⬜ TODO

## Phase
Phase 7 — `agh-querylog`

## Dependencies
- TASK-06 ✅ (`QueryLogConfig`)

## Objective
Implement the append-only query log file storage with rotation. Port from `internal/querylog/querylogfile.go`. Each DNS query is logged as a JSON-lines entry.

---

## Checklist

- [ ] Create `src/storage.rs`:
  ```rust
  pub struct QueryLogStorage {
      config:  QueryLogConfig,
      current: Arc<Mutex<BufWriter<File>>>,
      path:    PathBuf,
  }

  impl QueryLogStorage {
      pub fn open(dir: &Path, config: QueryLogConfig) -> Result<Self>;
      pub async fn append(&self, entry: QueryLogEntry) -> Result<()>;
      pub fn rotate(&self) -> Result<()>;  // rename .json → .json.1, open fresh .json
  }

  #[derive(Serialize, Deserialize, Debug, Clone)]
  pub struct QueryLogEntry {
      pub t:    DateTime<Utc>,           // "T" RFC3339 timestamp
      pub q:    String,                  // "QH" query hostname
      pub qtype: u16,                    // "QT" query type (A=1, AAAA=28, etc.)
      pub a:    String,                  // "Answer" — resolved IP(s)
      pub client: IpAddr,
      pub client_name: Option<String>,
      pub elapsed_ms: f64,
      pub upstream: Option<String>,
      pub result: QueryResult,
  }

  #[derive(Serialize, Deserialize)]
  pub struct QueryResult {
      pub reason: u8,             // 0=NotFiltered, 1=FilteredBlackList, etc.
      pub filter_id: Option<u64>,
      pub rule: Option<String>,
  }
  ```

- [ ] Storage files: `querylog.json` (current) + `querylog.json.1` (previous after rotation)
- [ ] Rotation trigger: when `querylog.json` exceeds `config.size_mb` megabytes
- [ ] Flush every 500ms (not on every write — batch for performance)
- [ ] IP anonymization: if `config.anonymize_client_ip`, mask last octet (`1.2.3.x`)

---

## Tests

```rust
#[tokio::test]
async fn test_append_and_rotation() {
    // append entries until size limit exceeded, verify rotation occurred
}

#[tokio::test]
async fn test_ip_anonymization() { ... }
```

---

## Verification
```bash
cargo test -p agh-querylog storage
```

---

## Output Files
- `rust-port/crates/agh-querylog/src/storage.rs`
- Update `PROGRESS.md`: TASK-28 → ✅ DONE
