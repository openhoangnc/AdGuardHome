# TASK-38: `agh-updater` — Download & Atomic Binary Replace

## Status
⬜ TODO

## Phase
Phase 9 — `agh-updater`

## Dependencies
- TASK-37 ✅ (VersionChecker — download URL from VersionInfo)

## Objective
Implement the actual binary download, checksum verification, tarball extraction, and atomic replacement of the running binary.

---

## Checklist

- [ ] Create `src/download.rs`:
  ```rust
  pub struct Updater {
      work_dir: PathBuf,
      http: reqwest::Client,
  }

  impl Updater {
      pub async fn download_and_apply(&self, info: &VersionInfo) -> Result<(), UpdaterError>;
  }
  ```

- [ ] Download logic:
  1. Determine download URL from `VersionInfo` + current OS/arch
     - URL pattern: `https://static.adguard.com/adguardhome/{channel}/AdGuardHome_{os}_{arch}.tar.gz`
  2. Stream download to `work_dir/agh-update.tar.gz.tmp`
  3. Compute SHA256 checksum as it streams (via `sha2::Sha256`)
  4. Verify checksum against checksum file (download `...sha256.txt` separately)
  5. Extract tarball (`flate2` + `tar`) to `work_dir/agh-update/`
  6. Find `AdGuardHome` binary in extracted directory
  7. Set executable bit (`std::os::unix::fs::PermissionsExt`)
  8. Atomic rename: `rename(agh-update/AdGuardHome, current_binary_path)`
     - `current_binary_path` = `std::env::current_exe()?`
  9. On Windows: rename current binary to `.old`, rename new binary to current name
  10. Log all steps with `tracing::info!`

- [ ] Cleanup: remove `work_dir/agh-update/` on success or failure
- [ ] After successful binary replacement, the caller (TASK-40) should exec the new binary (or exit and let the init system restart)

---

## Tests

```rust
#[tokio::test]
async fn test_checksum_verification_ok() {
    // Use a small test archive with known checksum
}

#[tokio::test]
async fn test_wrong_checksum_rejected() { ... }

#[tokio::test]
async fn test_extraction_finds_binary() { ... }
```

---

## Verification
```bash
cargo test -p agh-updater download
```

---

## Output Files
- `rust-port/crates/agh-updater/src/download.rs`
- Update `PROGRESS.md`: TASK-38 → ✅ DONE
