# TASK-36: `agh-web` — TLS Configuration & HTTPS

## Status
⬜ TODO

## Phase
Phase 8 — `agh-web`

## Dependencies
- TASK-33 ✅ (Router base)

## Objective
Implement TLS/HTTPS termination for the web UI and API. The Rust binary must support the same TLS configuration as the Go binary. Port from `internal/home/tls.go`.

---

## Checklist

- [ ] Create `src/tls_config.rs`:
  ```rust
  pub fn load_tls_config(tls: &TlsConfig) -> Result<rustls::ServerConfig, TlsError> {
      // Load cert chain from TlsConfig.certificate_chain (PEM or base64)
      // Load private key from TlsConfig.private_key
      // Return rustls::ServerConfig with TLS 1.2 min
  }
  ```

- [ ] HTTPS server setup in main (TASK-40): if `tls.enabled = true`:
  - Wrap axum with `tokio-rustls` `TlsAcceptor`
  - Bind on `tls.port_https` (default 443)
  - Optionally redirect HTTP → HTTPS if configured

- [ ] TLS API routes (`src/routes/tls.rs`):
  - [ ] `GET /control/tls/status` — return current TLS config + certificate validity info:
    ```json
    {
      "enabled": true,
      "server_name": "home.example.com",
      "force_https": false,
      "port_https": 443,
      "port_dns_over_tls": 853,
      "port_dns_over_quic": 853,
      "certificate_chain": "...",
      "private_key": "...",
      "certificate_domain": "home.example.com",
      "valid_cert": true,
      "valid_key": true,
      "valid_pair": true,
      "not_after": "2026-12-31T23:59:59Z",
      "warning_validation": ""
    }
    ```
  - [ ] `POST /control/tls/configure` — update TLS config, reload certificate
  - [ ] `POST /control/tls/validate` — same as configure but **don't save** — just validate cert/key pair

- [ ] Certificate validation:
  - Parse PEM/base64 cert chain
  - Verify private key matches public key in cert
  - Check cert `NotAfter` expiry
  - Verify domain matches `TlsConfig.server_name`
  - Return `valid_cert`, `valid_key`, `valid_pair` booleans

---

## Tests

```rust
#[test]
fn test_cert_validation_success() {
    // Generate self-signed cert with rcgen, validate it
}

#[test]
fn test_cert_expired_invalid() { ... }

#[test]
fn test_key_mismatch_invalid() { ... }
```

---

## Verification
```bash
cargo test -p agh-web tls
```

---

## Output Files
- `rust-port/crates/agh-web/src/tls_config.rs`
- `rust-port/crates/agh-web/src/routes/tls.rs`
- Update `PROGRESS.md`: TASK-36 → ✅ DONE
