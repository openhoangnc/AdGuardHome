//! DHCPv4 server stub (RFC 2131).
//!
//! TODO: Implement using raw UDP sockets via `socket2`.
//! Requires binding to 0.0.0.0:67 (broadcast) and 255.255.255.255.

/// DHCPv4 server configuration.
pub struct DhcpV4Config {
    pub interface: String,
    pub gateway: std::net::Ipv4Addr,
    pub subnet_mask: std::net::Ipv4Addr,
    pub range_start: std::net::Ipv4Addr,
    pub range_end: std::net::Ipv4Addr,
    pub lease_duration: std::time::Duration,
}
