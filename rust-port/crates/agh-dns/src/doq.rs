//! DNS-over-QUIC (DoQ) — RFC 9250.
//!
//! TODO: Implement using the `quinn` QUIC crate once the TLS configuration
//! (TASK-36) is available. The server would listen on UDP and use QUIC
//! to multiplex DNS queries over a single connection.
