//! DHCPv6 server — RFC 3315 / RFC 8415.
//!
//! Listens on UDP port 547 and joins the DHCPv6 all-servers multicast group
//! `ff02::1:2`. Handles SOLICIT (→ ADVERTISE) and REQUEST (→ REPLY) messages.

use std::net::{Ipv6Addr, SocketAddrV6, UdpSocket as StdUdpSocket};
use std::sync::Arc;
use std::time::Duration;

use socket2::{Domain, Protocol, Socket, Type};

use crate::leases::{Lease, LeaseStore};

/// DHCPv6 server port (RFC 3315).
pub const DHCPV6_SERVER_PORT: u16 = 547;
/// DHCPv6 all-servers multicast address (link-scoped, RFC 3315 §5.1).
const ALL_DHCP_SERVERS: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 1, 2);

/// DHCPv6 message types (RFC 8415 §7.3).
const SOLICIT: u8 = 1;
const ADVERTISE: u8 = 2;
const REQUEST: u8 = 3;
const REPLY: u8 = 7;

/// Configuration for the DHCPv6 server.
pub struct DhcpV6Config {
    pub interface_index: u32,
    pub range_start: Ipv6Addr,
    pub range_end: Ipv6Addr,
    pub lease_duration: Duration,
}

/// DHCPv6 server.
pub struct DhcpV6Server {
    config: DhcpV6Config,
    leases: Arc<LeaseStore>,
}

impl DhcpV6Server {
    pub fn new(config: DhcpV6Config, leases: Arc<LeaseStore>) -> Self {
        Self { config, leases }
    }

    /// Start the DHCPv6 server. Binds to `[::]:547` and joins multicast.
    pub async fn run(&self) -> Result<(), crate::DhcpError> {
        let socket = build_dhcpv6_socket(self.config.interface_index)?;
        tracing::info!("DHCPv6 server listening on [::]:547");

        let mut buf = [0u8; 1500];
        loop {
            let (len, src) = socket.recv_from(&mut buf).map_err(crate::DhcpError::Io)?;
            if len < 4 {
                continue;
            }
            let pkt = &buf[..len];
            let msg_type = pkt[0];

            // Extract transaction ID (bytes 1–3).
            let xid = [pkt[1], pkt[2], pkt[3]];

            match msg_type {
                SOLICIT => {
                    // Allocate an IPv6 address.
                    let duid = extract_client_duid(&pkt[4..]);
                    let ip = self.allocate_v6(&duid).await;
                    if let Some(ip) = ip {
                        let reply = build_v6_reply(ADVERTISE, xid, ip, &self.config);
                        let _ = socket.send_to(&reply, src);
                    }
                }
                REQUEST => {
                    let duid = extract_client_duid(&pkt[4..]);
                    let ip = self.allocate_v6(&duid).await;
                    if let Some(ip) = ip {
                        let reply = build_v6_reply(REPLY, xid, ip, &self.config);
                        let _ = socket.send_to(&reply, src);
                        self.record_v6_lease(&duid, ip).await;
                    }
                }
                _ => {}
            }
        }
    }

    async fn allocate_v6(&self, duid: &str) -> Option<Ipv6Addr> {
        // Reuse existing lease.
        if let Some(lease) = self.leases.find_by_mac(duid).await {
            if let Ok(ip) = lease.ip.parse::<Ipv6Addr>() {
                return Some(ip);
            }
        }
        // Find a free address in range.
        let start = u128::from(self.config.range_start);
        let end = u128::from(self.config.range_end);
        for addr in start..=end {
            let ip = Ipv6Addr::from(addr);
            if !self.leases.is_ip_taken(&ip.to_string()).await {
                return Some(ip);
            }
        }
        None
    }

    async fn record_v6_lease(&self, duid: &str, ip: Ipv6Addr) {
        let expires = chrono::Utc::now()
            + chrono::Duration::from_std(self.config.lease_duration)
                .unwrap_or(chrono::Duration::hours(24));
        let lease = Lease {
            mac: duid.to_owned(),
            ip: ip.to_string(),
            hostname: String::new(),
            expires,
            is_static: false,
        };
        let _ = self.leases.add_or_update(lease).await;
    }
}

/// Create a UDP socket suitable for DHCPv6 (joins ff02::1:2 multicast).
pub fn build_dhcpv6_socket(interface_index: u32) -> Result<StdUdpSocket, crate::DhcpError> {
    let socket = Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP))
        .map_err(crate::DhcpError::Io)?;
    socket
        .set_reuse_address(true)
        .map_err(crate::DhcpError::Io)?;
    socket.set_only_v6(true).map_err(crate::DhcpError::Io)?;

    let bind_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, DHCPV6_SERVER_PORT, 0, 0);
    socket
        .bind(&bind_addr.into())
        .map_err(crate::DhcpError::Io)?;

    // Join the all-DHCP-servers multicast group on the specified interface.
    let _multicast_req = socket2::InterfaceIndexOrAddress::Index(interface_index);
    socket
        .join_multicast_v6(&ALL_DHCP_SERVERS, interface_index)
        .map_err(crate::DhcpError::Io)?;

    let udp: StdUdpSocket = socket.into();
    udp.set_nonblocking(false).map_err(crate::DhcpError::Io)?;
    Ok(udp)
}

/// Extract client DUID from DHCPv6 options (option code 1).
fn extract_client_duid(options: &[u8]) -> String {
    let mut i = 0;
    while i + 3 < options.len() {
        let code = u16::from_be_bytes([options[i], options[i + 1]]);
        let len = u16::from_be_bytes([options[i + 2], options[i + 3]]) as usize;
        if code == 1 && i + 4 + len <= options.len() {
            return hex::encode(&options[i + 4..i + 4 + len]);
        }
        i += 4 + len;
    }
    String::new()
}

/// Build a minimal DHCPv6 ADVERTISE or REPLY packet.
fn build_v6_reply(msg_type: u8, xid: [u8; 3], addr: Ipv6Addr, cfg: &DhcpV6Config) -> Vec<u8> {
    let mut pkt = Vec::with_capacity(64);
    pkt.push(msg_type);
    pkt.extend_from_slice(&xid);

    // Option 3: IA_NA (Identity Association for Non-temporary Addresses)
    // Option code 3, followed by IAID + T1 + T2 + sub-option 5 (IA Address).
    let iaid = [0u8; 4];
    let t1 = (cfg.lease_duration.as_secs() / 2) as u32;
    let t2 = (cfg.lease_duration.as_secs() * 4 / 5) as u32;

    // IA Address option (option 5): addr (16 bytes) + preferred (4) + valid (4)
    let preferred = cfg.lease_duration.as_secs() as u32;
    let valid = preferred;
    let mut iaaddr_opt = Vec::new();
    iaaddr_opt.extend_from_slice(&5u16.to_be_bytes()); // option 5
    iaaddr_opt.extend_from_slice(&24u16.to_be_bytes()); // length: 16+4+4
    iaaddr_opt.extend_from_slice(&addr.octets());
    iaaddr_opt.extend_from_slice(&preferred.to_be_bytes());
    iaaddr_opt.extend_from_slice(&valid.to_be_bytes());

    let ia_na_data_len = (4 + 4 + 4 + iaaddr_opt.len()) as u16; // IAID+T1+T2+iaaddr
    pkt.extend_from_slice(&3u16.to_be_bytes()); // option 3: IA_NA
    pkt.extend_from_slice(&ia_na_data_len.to_be_bytes());
    pkt.extend_from_slice(&iaid);
    pkt.extend_from_slice(&t1.to_be_bytes());
    pkt.extend_from_slice(&t2.to_be_bytes());
    pkt.extend_from_slice(&iaaddr_opt);

    pkt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_client_duid_present() {
        // Build an options block with option code 1 (DUID), length 4, data [aa bb cc dd]
        let options = [0u8, 1, 0, 4, 0xaa, 0xbb, 0xcc, 0xdd];
        let duid = extract_client_duid(&options);
        assert_eq!(duid, "aabbccdd");
    }

    #[test]
    fn test_extract_client_duid_absent() {
        let options = [0u8, 2, 0, 4, 1, 2, 3, 4]; // option 2, not option 1
        let duid = extract_client_duid(&options);
        assert_eq!(duid, "");
    }

    #[test]
    fn test_build_v6_reply_message_type() {
        let cfg = DhcpV6Config {
            interface_index: 1,
            range_start: Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1),
            range_end: Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 0xfffe),
            lease_duration: Duration::from_secs(86400),
        };
        let addr = Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1);
        let pkt = build_v6_reply(ADVERTISE, [1, 2, 3], addr, &cfg);
        assert_eq!(pkt[0], ADVERTISE);
        assert_eq!(&pkt[1..4], &[1, 2, 3]); // transaction ID
    }

    #[test]
    fn test_build_v6_reply_contains_ia_na() {
        let cfg = DhcpV6Config {
            interface_index: 1,
            range_start: Ipv6Addr::UNSPECIFIED,
            range_end: Ipv6Addr::UNSPECIFIED,
            lease_duration: Duration::from_secs(3600),
        };
        let pkt = build_v6_reply(REPLY, [0, 0, 1], Ipv6Addr::LOCALHOST, &cfg);
        // Option 3 (IA_NA) should start at byte 4.
        assert_eq!(u16::from_be_bytes([pkt[4], pkt[5]]), 3);
    }
}
