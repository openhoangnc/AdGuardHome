# TASK-32: `agh-web` — Frontend Embedding & Serving

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-01 ✅ (crate skeletons exist)

## Objective
Embed the prebuilt frontend (`build/` directory) and serve it as a SPA at `/`. The frontend is **never modified** — only the serving mechanism is implemented in Rust.

---

## Checklist

- [ ] Create `src/frontend.rs`:
  ```rust
  // Embed the entire build/ directory at compile time
  #[derive(RustEmbed)]
  #[folder = "../../build"]   // relative to crate root
  #[prefix = ""]
  struct ClientAssets;

  /// Axum route handler for frontend assets
  pub async fn serve_asset(uri: Uri) -> impl IntoResponse {
      let path = uri.path().trim_start_matches('/');

      // Try exact path first
      if let Some(asset) = ClientAssets::get(path) {
          return Response::builder()
              .header("Content-Type", mime_type(path))
              .header("Cache-Control", "public, max-age=31536000, immutable")  // for hashed assets
              .body(Body::from(asset.data))
              .unwrap();
      }

      // SPA fallback: serve index.html for any non-asset path
      let index = ClientAssets::get("index.html").unwrap();
      Response::builder()
          .header("Content-Type", "text/html; charset=utf-8")
          .header("Cache-Control", "no-cache")
          .body(Body::from(index.data))
          .unwrap()
  }
  ```

- [ ] Serve at `/` with the `GET /*` fallback (must not intercept `/control/` — API routes take priority)
- [ ] Cache headers:
  - Hashed assets (`*.js`, `*.css` with hash in filename): `max-age=31536000, immutable`
  - `index.html`: `no-cache`
  - Fonts, images: `max-age=604800`
- [ ] `--local-frontend` CLI flag: if set, serve from disk path instead of embedded (for dev mode)
- [ ] Correct MIME types for all asset types (`.js`, `.css`, `.wasm`, `.svg`, `.png`, `.ico`, etc.)
- [ ] Gzip pre-compressed support: if `build/` contains `.gz` files, serve them with `Content-Encoding: gzip`

---

## Tests

```rust
#[tokio::test]
async fn test_index_html_served_at_root() {
    let response = client.get("/").send().await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(response.headers()["content-type"].to_str().unwrap().contains("text/html"));
}

#[tokio::test]
async fn test_spa_fallback_on_unknown_path() {
    let response = client.get("/some/spa/route").send().await.unwrap();
    assert_eq!(response.status(), 200); // returns index.html
}

#[tokio::test]
async fn test_api_not_intercepted() {
    let response = client.get("/control/status").send().await.unwrap();
    // Should not be intercepted by frontend handler — let API handler respond
    assert_ne!(response.headers()["content-type"].to_str().unwrap(), "text/html; charset=utf-8");
}
```

---

## Verification
```bash
cargo test -p agh-web frontend
```

---

## Output Files
- `rust-port/crates/agh-web/src/frontend.rs`
- Update `PROGRESS.md`: TASK-32 → ✅ DONE
