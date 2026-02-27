# TASK-44: DNS Compliance Tests

## Status
⬜ TODO

## Phase
Phase 12 — Testing

## Dependencies
- TASK-20 ✅ (Full filtering pipeline wired)

## Objective
Run DNS protocol compliance tests against the Rust DNS server using established DNS testing tools.

---

## Checklist

- [ ] Install test tools:
  ```bash
  brew install q     # DNS query tool with DoH/DoT/DoQ support (macOS)
  # or: go install github.com/natesales/q@latest
  ```

- [ ] Plain DNS tests (via `dig` against port 5353 in test mode):
  - [ ] A record resolution
  - [ ] AAAA record resolution
  - [ ] CNAME record resolution
  - [ ] MX record resolution
  - [ ] TXT record resolution
  - [ ] PTR record for blocked domain
  - [ ] NXDOMAIN for non-existent domain
  - [ ] Wildcard domain block

- [ ] DoH tests:
  ```bash
  q @https://localhost:8443/dns-query google.com
  q @https://localhost:8443/dns-query ads.example.com  # should return 0.0.0.0
  ```

- [ ] DoT tests:
  ```bash
  q @tls://localhost:8853 google.com
  ```

- [ ] Write automated test script `rust-port/tests/dns_compliance.sh`:
  - Start `adguardhome` in test mode on high ports (5353 DNS, 8443 DoH, 8853 DoT)
  - Run `dig` / `q` queries and check outputs
  - Exit 0 if all pass, 1 if any fail

- [ ] Compare responses between Go and Rust binaries for the same queries:
  ```bash
  # Run Go binary, capture responses:
  dig @127.0.0.1 -p 5353 google.com > go_response.txt
  # Run Rust binary, capture responses:
  dig @127.0.0.1 -p 5354 google.com > rust_response.txt
  diff go_response.txt rust_response.txt  # should be equivalent
  ```

---

## Acceptance Criteria
- All plain DNS queries return correct responses
- DoH queries work (HTTP 200, correct DNS wire format response)
- DoT queries work (TLS handshake succeeds, responses correct)
- Blocked domains return `0.0.0.0` or NXDOMAIN as configured
- Response times <5ms for cached queries

---

## Verification
```bash
bash rust-port/tests/dns_compliance.sh
```

---

## Output Files
- `rust-port/tests/dns_compliance.sh`
- Update `PROGRESS.md`: TASK-44 → ✅ DONE
