# TASK-19: `agh-dns` — DNS-over-QUIC (DoQ) + DNSCrypt

## Status
⬜ TODO

## Phase
Phase 4 — `agh-dns`

## Dependencies
- TASK-15 ✅ (Core DNS server — `AdGuardRequestHandler` reused)

## Objective
Implement DoQ and DNSCrypt servers. These are advanced encrypted DNS protocols.

> ⚠️ **Risk**: `hickory-dns` doesn't fully implement DoQ or DNSCrypt. If gaps are found, document them and use a workaround strategy (C FFI via `libdnscrypt` for DNSCrypt, or wrap the `quinn` QUIC library directly for DoQ). Update `PROGRESS.md` with the decision.

---

## Checklist

### DNS-over-QUIC (DoQ) — RFC 9250

- [ ] Create `src/doq.rs`:
  ```rust
  pub struct DoqServer {
      endpoint: quinn::Endpoint,
      handler:  Arc<AdGuardRequestHandler>,
  }
  impl DoqServer {
      pub async fn new(tls_config: Arc<ServerConfig>, port: u16 /* default 853/UDP */) -> Self;
      pub async fn run(self) -> Result<()>;
  }
  ```
- [ ] Accept QUIC connections (`quinn::Incoming`)
- [ ] Per connection: accept bidirectional streams
- [ ] Read DNS message from stream (2-byte length-prefix per RFC 9250 §4.2)
- [ ] Process through `AdGuardRequestHandler`
- [ ] Write response back to stream
- [ ] Handle connection errors gracefully

### DNSCrypt

- [ ] Create `src/dnscrypt.rs`
- [ ] Research: check if the `dnscrypt` crate on crates.io is maintained and suitable
- [ ] If no suitable crate: document this as a known gap and mark the task `🔴 BLOCKED` with details
- [ ] If crate exists: wrap it and route through `AdGuardRequestHandler`
- [ ] Generate DNSCrypt provider keys and embed in config (`TlsConfig` extension)

---

## Tests

```rust
#[tokio::test]
async fn test_doq_dns_query() {
    // Connect with quinn QUIC client, send DNS query, verify response
}
```

---

## Verification
```bash
cargo test -p agh-dns doq
# Manual (requires q tool or dnslookup):
q @quic://127.0.0.1:8853 google.com
```

---

## Output Files
- `rust-port/crates/agh-dns/src/doq.rs`
- `rust-port/crates/agh-dns/src/dnscrypt.rs`
- Update `PROGRESS.md`: TASK-19 → ✅ DONE (or 🔴 BLOCKED with reason)
