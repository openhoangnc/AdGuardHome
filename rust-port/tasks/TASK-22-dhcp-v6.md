# TASK-22: `agh-dhcp` — DHCPv6 Server

## Status
⬜ TODO

## Phase
Phase 5 — `agh-dhcp`

## Dependencies
- TASK-21 ✅ (DHCPv4 — lease store architecture established)

## Objective
Implement a DHCPv6 (RFC 3315) server. Port from `internal/dhcpd/v6.go`.

---

## Checklist

- [ ] Create `src/v6.rs`:
  ```rust
  pub struct DhcpV6Server {
      config: DhcpV6Config,
      leases: Arc<Mutex<LeaseStore>>,
  }
  ```
- [ ] Bind to `[::]:547` (DHCPv6 server port), join multicast group `ff02::1:2` on the interface
- [ ] Parse DHCPv6 messages (fixed 4-byte header + options)
- [ ] Implement Solicit → Advertise → Request → Reply state machine
- [ ] Assign IPv6 addresses from configured prefix (e.g., `2001:db8::/64`)
- [ ] Generate DUID (DHCP Unique Identifier) for the server
- [ ] Options: IA_NA (Identity Association for Non-temporary Address), DNS Recursive Name Server

---

## Tests

```rust
#[tokio::test]
async fn test_v6_solicit_advertise() { ... }
```

---

## Verification
```bash
cargo test -p agh-dhcp v6
```

---

## Output Files
- `rust-port/crates/agh-dhcp/src/v6.rs`
- Update `PROGRESS.md`: TASK-22 → ✅ DONE
