# TASK-23: `agh-dhcp` — Lease Persistence & ARP Table

## Status
⬜ TODO

## Phase
Phase 5 — `agh-dhcp`

## Dependencies
- TASK-21 ✅ (DHCPv4 uses LeaseStore)

## Objective
Implement persistent lease storage and ARP-based client discovery. Port from `internal/dhcpd/leases.go` and `internal/dhcpd/arp.go`.

---

## Checklist

### Lease Persistence

- [ ] Create `src/leases.rs`:
  ```rust
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct Lease {
      pub ip:       IpAddr,
      pub mac:      MacAddr,
      pub hostname: String,
      pub expiry:   DateTime<Utc>,    // None = static lease
      pub is_static: bool,
  }

  pub struct LeaseStore {
      leases:   Vec<Lease>,
      path:     PathBuf,
  }

  impl LeaseStore {
      pub fn load(path: &Path) -> Result<Self>;
      pub fn save(&self) -> Result<()>;
      pub fn add(&mut self, lease: Lease);
      pub fn remove_by_mac(&mut self, mac: &MacAddr);
      pub fn find_by_ip(&self, ip: &IpAddr) -> Option<&Lease>;
      pub fn find_by_mac(&self, mac: &MacAddr) -> Option<&Lease>;
      pub fn expired(&self) -> Vec<&Lease>;
  }
  ```
- [ ] Storage format: JSON file `leases.json` (same name as Go for compatibility)
- [ ] Atomic write (`.tmp` + rename pattern — same as config)
- [ ] Save on every allocation, release, and static lease change
- [ ] Expire old leases: run cleanup task every 60 seconds, remove expired dynamic leases

### ARP Table

- [ ] Create `src/arp.rs`:
  ```rust
  pub async fn read_arp_table() -> Vec<(IpAddr, MacAddr)>;
  ```
- [ ] Linux: parse `/proc/net/arp`
- [ ] macOS: parse output of `arp -an` (via `Command::new("arp")`)
- [ ] Use ARP data to auto-discover clients and provide named entries in the `ClientRegistry`

---

## Tests

```rust
#[test]
fn test_lease_roundtrip() {
    let store = LeaseStore { leases: vec![...], path: tmp_path() };
    store.save().unwrap();
    let loaded = LeaseStore::load(&tmp_path()).unwrap();
    assert_eq!(store.leases.len(), loaded.leases.len());
}

#[test]
fn test_expired_leases_removed() { ... }

#[cfg(target_os = "linux")]
#[test]
fn test_arp_table_parse() {
    let content = "192.168.1.1 0x1 0x2 00:11:22:33:44:55 * eth0\n";
    let entries = parse_arp_table(content);
    assert_eq!(entries.len(), 1);
}
```

---

## Verification
```bash
cargo test -p agh-dhcp leases arp
```

---

## Output Files
- `rust-port/crates/agh-dhcp/src/leases.rs`
- `rust-port/crates/agh-dhcp/src/arp.rs`
- Update `PROGRESS.md`: TASK-23 → ✅ DONE
