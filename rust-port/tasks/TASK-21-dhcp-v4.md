# TASK-21: `agh-dhcp` — DHCPv4 Server

## Status
⬜ TODO

## Phase
Phase 5 — `agh-dhcp`

## Dependencies
- TASK-06 ✅ (`DhcpConfig` from agh-core)

## Objective
Implement a DHCPv4 (RFC 2131) server using raw UDP sockets. Port from `internal/dhcpd/v4.go`.

---

## Checklist

- [ ] Create `src/v4.rs`:
  ```rust
  pub struct DhcpV4Server {
      config: DhcpV4Config,
      leases: Arc<Mutex<LeaseStore>>,
      socket: socket2::Socket,
  }

  impl DhcpV4Server {
      pub async fn new(config: DhcpV4Config, leases: Arc<Mutex<LeaseStore>>) -> Result<Self>;
      pub async fn run(self) -> Result<()>;
  }
  ```

- [ ] Bind raw UDP socket to port 67 on the configured interface (use `socket2` for `SO_BROADCAST`)
- [ ] Send on port 68 (client port) using `socket2`
- [ ] Implement DHCP state machine:
  - DISCOVER → OFFER (assign candidate IP from pool, cache as pending)
  - REQUEST → ACK (confirm lease, save to `LeaseStore`) or NAK (if IP taken)
  - RELEASE → remove lease
  - INFORM → ACK without IP assignment (client already has IP)
  - DECLINE → mark IP as unavailable temporarily
- [ ] Parse DHCP packets: fixed header (240 bytes) + options (TLV format)
- [ ] Generate OFFER/ACK packets with options:
  - `53` (message type), `54` (server identifier), `51` (lease time)
  - `1` (subnet mask), `3` (router), `6` (DNS servers → point to `agh-dns`)
  - `15` (domain name - optional)
- [ ] IP pool management: allocate from `range_start` to `range_end`, skip reserved and taken IPs
- [ ] Platform support:
  - Linux: bind to `INADDR_ANY:67`, use `SO_BINDTODEVICE` for interface selection
  - macOS: bind to broadcast address of interface

---

## Tests

```rust
#[tokio::test]
async fn test_discover_offer_cycle() {
    // Send raw DISCOVER packet on loopback, expect OFFER response
}

#[tokio::test]
async fn test_request_ack() { ... }

#[tokio::test]
async fn test_ip_pool_exhaustion() { ... }
```

---

## Verification
```bash
cargo test -p agh-dhcp v4
# Platform note: raw socket tests may need sudo or capability on Linux
```

---

## Output Files
- `rust-port/crates/agh-dhcp/src/v4.rs`
- Update `PROGRESS.md`: TASK-21 → ✅ DONE
