# TASK-09: `agh-core` — Client Registry

## Status
⬜ TODO

## Phase
Phase 2 — `agh-core`

## Dependencies
- TASK-06 ✅ (config types)

## Objective
Implement the client registry — the in-memory store of persistent client configurations (IP → name, settings, per-client filtering rules). This is queried by `agh-dns` on every DNS request to apply per-client rules.

---

## Checklist

- [ ] Create `src/client.rs`:

  ```rust
  /// A persistent client configured by the admin
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct PersistentClient {
      pub name: String,
      pub ids: Vec<String>,          // IP address, MAC address, or CIDR
      pub tags: Vec<String>,
      pub blocked_services: Vec<String>,
      pub upstreams: Vec<String>,
      pub filtering_enabled: bool,
      pub parental_enabled: bool,
      pub safebrowsing_enabled: bool,
      pub safesearch: SafeSearchConfig,
      pub use_global_settings: bool,
      pub use_global_blocked_services: bool,
      pub ignore_querylog: bool,
      pub ignore_statistics: bool,
  }

  /// Runtime client info (auto-discovered from DNS/DHCP)
  #[derive(Debug, Clone)]
  pub struct RuntimeClient {
      pub ip: IpAddr,
      pub name: Option<String>,      // from rDNS or DHCP
      pub source: ClientSource,
  }

  pub enum ClientSource { Rdns, Dhcp, Arp, Hosts }

  pub struct ClientRegistry {
      persistent: Arc<RwLock<Vec<PersistentClient>>>,
      runtime: Arc<RwLock<HashMap<IpAddr, RuntimeClient>>>,
  }

  impl ClientRegistry {
      pub fn new(clients: Vec<PersistentClient>) -> Self;

      /// Find persistent client by IP/MAC/CIDR match
      pub fn find_persistent(&self, ip: &IpAddr) -> Option<PersistentClient>;

      pub fn find_runtime(&self, ip: &IpAddr) -> Option<RuntimeClient>;
      pub fn add_runtime(&self, client: RuntimeClient);

      pub fn list_persistent(&self) -> Vec<PersistentClient>;
      pub fn add_persistent(&self, client: PersistentClient) -> Result<(), ClientError>;
      pub fn remove_persistent(&self, name: &str) -> Result<(), ClientError>;
      pub fn update_persistent(&self, name: &str, client: PersistentClient) -> Result<(), ClientError>;
  }
  ```

- [ ] Implement CIDR matching for `ids`: if an id looks like `192.168.1.0/24`, match any IP in that range
- [ ] Implement MAC address matching (format: `AA:BB:CC:DD:EE:FF`)
- [ ] Add duplicate detection: two persistent clients cannot have overlapping `ids`

---

## Tests

```rust
#[test]
fn test_find_by_ip_exact() { ... }

#[test]
fn test_find_by_cidr() { ... }

#[test]
fn test_add_duplicate_id_rejected() { ... }
```

---

## Verification
```bash
cargo test -p agh-core client
```

---

## Output Files
- `rust-port/crates/agh-core/src/client.rs`
- Update `PROGRESS.md`: TASK-09 → ✅ DONE
