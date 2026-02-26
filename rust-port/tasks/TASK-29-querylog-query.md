# TASK-29: `agh-querylog` — Pagination & Filtering

## Status
⬜ TODO

## Phase
Phase 7 — `agh-querylog`

## Dependencies
- TASK-28 ✅ (QueryLogStorage)

## Objective
Implement cursor-based pagination and filtering over the query log for the `GET /control/querylog` endpoint.

---

## Checklist

- [ ] Create `src/query.rs`:
  ```rust
  pub struct QueryLogService {
      storage: Arc<QueryLogStorage>,
  }

  pub struct QueryLogParams {
      pub older_than: Option<DateTime<Utc>>,  // cursor for pagination
      pub limit:      usize,              // default 100, max 5000
      pub search:     Option<String>,     // filter by domain or client
      pub response_status: Option<ResponseStatus>,  // "filtered" | "processed" | "blocked_safebrowsing" | ...
      pub client:     Option<String>,     // filter by client IP
  }

  pub enum ResponseStatus { All, Filtered, Processed, SafeBrowsing, SafeSearch, Parental }

  pub struct QueryLogResponse {
      pub data:     Vec<QueryLogEntry>,
      pub oldest:   DateTime<Utc>,  // oldest log entry timestamp (for UI "Load more" cursor)
  }

  impl QueryLogService {
      pub async fn query(&self, params: QueryLogParams) -> Result<QueryLogResponse>;
  }
  ```

- [ ] Read from both `querylog.json` (current) and `querylog.json.1` (previous) in chronological-descending order
- [ ] Filter logic: apply `search` as substring match on domain + client IP; apply `response_status` on `result.reason`
- [ ] Cursor: `older_than` = skip entries newer than this timestamp → enables "load more" without re-fetching
- [ ] Apply limit after filtering

---

## Tests

```rust
#[tokio::test]
async fn test_paginated_query() {
    // Insert 200 entries, query with limit=100, get 100; then use oldest cursor, get next 100
}

#[tokio::test]
async fn test_domain_filter() { ... }

#[tokio::test]
async fn test_status_filter_blocked() { ... }
```

---

## Verification
```bash
cargo test -p agh-querylog query
```

---

## Output Files
- `rust-port/crates/agh-querylog/src/query.rs`
- Update `PROGRESS.md`: TASK-29 → ✅ DONE
