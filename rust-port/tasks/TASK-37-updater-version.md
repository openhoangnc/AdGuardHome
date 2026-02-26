# TASK-37: `agh-updater` — Version Check

## Status
⬜ TODO

## Phase
Phase 9 — `agh-updater`

## Dependencies
- TASK-06 ✅ (config types)

## Objective
Implement version checking against AdGuard's update server. Port from `internal/updater/version.go`.

---

## Checklist

- [ ] Create `src/version.rs`:
  ```rust
  pub struct VersionChecker {
      http: reqwest::Client,
      channel: UpdateChannel,
      current_version: &'static str,  // from build-time env or Cargo.toml version
  }

  #[derive(Serialize, Deserialize)]
  pub struct VersionInfo {
      pub version:          String,
      pub announcement:     String,
      pub announcement_url: String,
      #[serde(rename = "selfupdate_min_version")]
      pub selfupdate_min:   String,
      pub download_page:    String,
  }

  pub enum UpdateChannel { Release, Beta, Edge }

  impl VersionChecker {
      pub async fn check(&self) -> Result<Option<VersionInfo>, UpdaterError>;
      // Returns None if current version is already latest
  }
  ```

- [ ] URL: `https://static.adguard.com/adguardhome/{channel}/version.json`
- [ ] Compare: `semver` library to compare current vs. latest
- [ ] Cache result for 1 hour (don't hammer the update server)
- [ ] Detect current platform:
  - `cfg!(target_os = "linux")` → `"linux"`
  - `cfg!(target_os = "macos")` → `"darwin"`
  - `cfg!(target_os = "windows")` → `"windows"`
  - `cfg!(target_arch = "x86_64")` → `"amd64"`
  - `cfg!(target_arch = "aarch64")` → `"arm64"`

---

## Tests

```rust
#[tokio::test]
async fn test_version_parse() {
    let info: VersionInfo = serde_json::from_str(r#"{"version":"v0.108.0",...}"#).unwrap();
    assert_eq!(info.version, "v0.108.0");
}

#[tokio::test]
async fn test_no_update_when_current() { ... }
```

---

## Verification
```bash
cargo test -p agh-updater version
```

---

## Output Files
- `rust-port/crates/agh-updater/src/version.rs`
- Update `PROGRESS.md`: TASK-37 → ✅ DONE
