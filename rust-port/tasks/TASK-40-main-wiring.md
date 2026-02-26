# TASK-40: `agh-main` — Wire All Crates + Startup Sequence

## Status
⬜ TODO

## Phase
Phase 10 — `agh-main`

## Dependencies
- ALL previous crates ✅

## Objective
Implement `main.rs` — the binary entrypoint that initializes all subsystems in the correct order, handles signals (SIGTERM/SIGINT), and manages graceful shutdown.

---

## Checklist

- [ ] Implement `src/main.rs` startup sequence:

  ```
  1. Parse CLI args (agh-core::Cli::parse())
  2. init_tracing(verbose, logfile)
  3. ConfigManager::load(config_path) → config, is_first_run
  4. Handle service subcommands (TASK-39) → exit if service command
  5. Initialize ClientRegistry from config.clients
  6. Initialize FilteringEngine from config.filters + user_rules
  7. Start FilterUpdater scheduler
  8. Initialize StatsStorage + StatsService
  9. Initialize QueryLogStorage + QueryLogService
  10. Initialize DhcpV4Server (if config.dhcp.enabled)
  11. Initialize DhcpV6Server (if config.dhcp.enabled && v6 config present)
  12. Initialize DnsServer (with FilteringEngine + StatsService + QueryLogService + ClientRegistry)
  13. Build axum Router (agh-web::build_router with all services injected as State)
  14. Start HTTP listener (plain or TLS depending on config)
  15. Start DoT server (if port_dns_over_tls configured)
  16. Start DoQ server (if port_dns_over_quic configured)
  17. tracing::info!("AdGuardHome is ready");
  18. Wait for SIGTERM/SIGINT (tokio::signal)
  19. Graceful shutdown: stop accepting new connections, flush querylog/stats, save leases
  ```

- [ ] Use `tokio::select!` to wait on multiple servers simultaneously
- [ ] Shutdown timeout: 10 seconds, then force exit
- [ ] Implement `AppState` struct that holds `Arc<>` to all services (passed to axum as `State`)
- [ ] Capability handling on Linux: if port < 1024 needed and running non-root, advice to use `setcap cap_net_bind_service`

---

## Tests

```rust
#[tokio::test]
async fn test_startup_with_minimal_config() {
    // Start with test-config.yaml, verify all services initialize
    // Check /control/status returns 200
    // Shutdown gracefully
}
```

---

## Verification
```bash
cargo build -p agh-main
./target/debug/adguardhome --help
./target/debug/adguardhome -c rust-port/tests/fixtures/test-config.yaml -w /tmp/agh-test
# In another terminal:
curl http://localhost:3000/control/status
kill -TERM <pid>   # verify graceful shutdown
```

---

## Output Files
- `rust-port/crates/agh-main/src/main.rs`
- `rust-port/crates/agh-main/src/app_state.rs`
- Update `PROGRESS.md`: TASK-40 → ✅ DONE
