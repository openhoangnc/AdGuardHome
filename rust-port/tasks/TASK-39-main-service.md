# TASK-39: `agh-main` — System Service Management

## Status
⬜ TODO

## Phase
Phase 10 — `agh-main`

## Dependencies
- TASK-08 ✅ (CLI parsing — `ServiceCommand` enum)

## Objective
Implement system service management (install/uninstall/start/stop/status) for Linux (systemd), macOS (launchd), and Windows (SCM). Port from Go's `service` library usage in `internal/aghos/servicecmd_*.go`.

---

## Checklist

- [ ] Create `crates/agh-main/src/service.rs`:

  ```rust
  pub fn handle_service_command(action: ServiceAction, work_dir: &Path, config: &Path) -> Result<()>;
  ```

### Linux — systemd

- [ ] Generate `.service` unit file:
  ```ini
  [Unit]
  Description=AdGuardHome - DNS ad blocker
  After=network.target

  [Service]
  Type=simple
  ExecStart=/usr/local/bin/AdGuardHome -c %h/AdGuardHome.yaml -w %h --no-check-update
  WorkingDirectory=%h
  Restart=on-failure
  RestartSec=5s
  AmbientCapabilities=CAP_NET_BIND_SERVICE

  [Install]
  WantedBy=multi-user.target
  ```
- [ ] Install: write to `/etc/systemd/system/adguardhome.service`, run `systemctl daemon-reload && systemctl enable adguardhome`
- [ ] Uninstall: `systemctl stop`, `systemctl disable`, delete unit file
- [ ] Start/Stop/Restart/Status: delegate to `systemctl`

### macOS — launchd

- [ ] Generate `io.adguard.adguardhome.plist` for `~/Library/LaunchAgents/` (or `/Library/LaunchDaemons/` for root)
- [ ] Install: write plist, run `launchctl load`
- [ ] Uninstall: `launchctl unload`, delete plist
- [ ] Start/Stop: `launchctl start/stop io.adguard.adguardhome`

### Windows — SCM

- [ ] Use `windows-service` crate (add as platform-specific dep: `[target.'cfg(windows)'.dependencies]`)
- [ ] Install: `sc.exe create AdGuardHome binPath= "..."` or use crate API
- [ ] Start/Stop/Status: delegate to SCM

---

## Tests

```rust
#[cfg(target_os = "linux")]
#[test]
fn test_systemd_unit_file_content() {
    let content = generate_systemd_unit("/usr/local/bin/AdGuardHome", "/etc/AdGuardHome.yaml");
    assert!(content.contains("[Service]"));
    assert!(content.contains("CAP_NET_BIND_SERVICE"));
}
```

---

## Verification
```bash
cargo test -p agh-main service
# Manual:
sudo ./target/debug/adguardhome service install
sudo systemctl status adguardhome
```

---

## Output Files
- `rust-port/crates/agh-main/src/service.rs`
- Update `PROGRESS.md`: TASK-39 → ✅ DONE
