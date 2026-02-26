# TASK-07: `agh-core` — Config Read/Write & Atomic Operations

## Status
⬜ TODO

## Phase
Phase 2 — `agh-core`

## Dependencies
- TASK-06 ✅ (`AdGuardHomeConfig` struct complete)

## Objective
Implement safe config file I/O: reading with validation, atomic writing (`.tmp` + rename), and `firstRun` detection. This replicates the behavior of `internal/home/config.go` read/write functions.

---

## Checklist

- [ ] Create `src/config_io.rs` with:

  ```rust
  pub struct ConfigManager {
      path: PathBuf,
      config: Arc<RwLock<AdGuardHomeConfig>>,
  }

  impl ConfigManager {
      /// Load config from disk, or create default if not found (firstRun)
      pub async fn load(path: &Path) -> Result<Self, ConfigError>;

      /// Atomically write config: write to `path.tmp`, then rename
      pub async fn save(&self) -> Result<(), ConfigError>;

      /// Get a clone of the current config
      pub fn get(&self) -> AdGuardHomeConfig;

      /// Update config in memory and persist to disk atomically
      pub async fn update<F>(&self, f: F) -> Result<(), ConfigError>
      where F: FnOnce(&mut AdGuardHomeConfig);

      /// Returns true if this is the first run (config file did not exist)
      pub fn is_first_run(&self) -> bool;
  }
  ```

- [ ] Implement `firstRun` detection: if config file doesn't exist → set `is_first_run = true`, use `AdGuardHomeConfig::default()`
- [ ] Implement atomic write:
  ```rust
  async fn write_atomic(path: &Path, content: &str) -> Result<(), ConfigError> {
      let tmp = path.with_extension("yaml.tmp");
      tokio::fs::write(&tmp, content).await?;
      tokio::fs::rename(&tmp, path).await?;
      Ok(())
  }
  ```
- [ ] Validate `schema_version` on load — if the version is newer than supported, return an informative error
- [ ] Add file watching hook (stubbed — implementation in TASK-10 `notify` crate usage)

---

## Tests

```rust
#[tokio::test]
async fn test_atomic_write_creates_file() { ... }

#[tokio::test]
async fn test_load_missing_file_is_first_run() { ... }

#[tokio::test]
async fn test_roundtrip_preserves_all_fields() {
    // Write config, read back, compare
}

#[tokio::test]
async fn test_concurrent_updates_are_safe() {
    // Spawn 10 tasks that update config simultaneously
}
```

---

## Error Type

```rust
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("Unsupported schema version: {0}")]
    UnsupportedVersion(u32),
}
```

---

## Verification
```bash
cd rust-port
cargo test -p agh-core config_io
```

---

## Output Files
- `rust-port/crates/agh-core/src/config_io.rs`
- Update `PROGRESS.md`: TASK-07 → ✅ DONE
