//! DHCPv4 server — RFC 2131.
//!
//! Implements a minimal DHCPv4 server using raw UDP sockets via `socket2`.
//! The server binds to the DHCP server port (67) with SO_BROADCAST and
//! processes DISCOVER, REQUEST, and INFORM messages.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket as StdUdpSocket};
use std::sync::Arc;
use std::time::{Duration, Instant};

use socket2::{Domain, Protocol, Socket, Type};
use tokio::sync::RwLock;

use crate::leases::{Lease, LeaseStore};

/// DHCPv4 message type option values (option 53).
const DHCP_DISCOVER: u8 = 1;
const DHCP_OFFER: u8 = 2;
const DHCP_REQUEST: u8 = 3;
const DHCP_ACK: u8 = 5;
const DHCP_NAK: u8 = 6;
const DHCP_INFORM: u8 = 8;

/// Fixed DHCP packet field offsets (RFC 2131 §2).
const BOOTP_OP: usize = 0;
const BOOTP_HTYPE: usize = 1;
const BOOTP_HLEN: usize = 2;
const BOOTP_XID: usize = 4;
const BOOTP_FLAGS: usize = 10;
const BOOTP_CIADDR: usize = 12;
const BOOTP_YIADDR: usize = 16;
const BOOTP_SIADDR: usize = 20;
const BOOTP_CHADDR: usize = 28;
const BOOTP_OPTIONS: usize = 236;
const DHCP_MAGIC_COOKIE: [u8; 4] = [99, 130, 83, 99];

/// Configuration for the DHCPv4 server.
pub struct DhcpV4Config {
    pub interface: String,
    pub gateway: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
    pub range_start: Ipv4Addr,
    pub range_end: Ipv4Addr,
    pub lease_duration: Duration,
}

/// DHCPv4 server state.
pub struct DhcpV4Server {
    config: DhcpV4Config,
    leases: Arc<LeaseStore>,
}

impl DhcpV4Server {
    pub fn new(config: DhcpV4Config, leases: Arc<LeaseStore>) -> Self {
        Self { config, leases }
    }

    /// Start the DHCPv4 server. Binds to 0.0.0.0:67 with SO_BROADCAST.
    ///
    /// Returns an error if the socket cannot be created (e.g., insufficient
    /// privileges to bind port 67).
    pub async fn run(&self) -> Result<(), crate::DhcpError> {
        let socket = build_dhcp_socket("0.0.0.0:67")?;
        tracing::info!("DHCPv4 server listening on 0.0.0.0:67");

        let mut buf = [0u8; 1500];
        loop {
            let (len, src) = socket.recv_from(&mut buf).map_err(crate::DhcpError::Io)?;
            if len < BOOTP_OPTIONS + 4 {
                continue;
            }
            let pkt = &buf[..len];

            // Verify magic cookie.
            if pkt[BOOTP_OPTIONS..BOOTP_OPTIONS + 4] != DHCP_MAGIC_COOKIE {
                continue;
            }

            // Parse DHCP message type from options.
            let msg_type = match parse_message_type(&pkt[BOOTP_OPTIONS + 4..]) {
                Some(t) => t,
                None => continue,
            };

            // Extract client MAC (chaddr, first 6 bytes).
            let mac = format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                pkt[BOOTP_CHADDR],
                pkt[BOOTP_CHADDR + 1],
                pkt[BOOTP_CHADDR + 2],
                pkt[BOOTP_CHADDR + 3],
                pkt[BOOTP_CHADDR + 4],
                pkt[BOOTP_CHADDR + 5],
            );

            // XID for correlating replies.
            let xid = [
                pkt[BOOTP_XID],
                pkt[BOOTP_XID + 1],
                pkt[BOOTP_XID + 2],
                pkt[BOOTP_XID + 3],
            ];

            match msg_type {
                DHCP_DISCOVER => {
                    let offer_ip = self.allocate_ip(&mac).await;
                    if let Some(ip) = offer_ip {
                        let reply = build_reply(DHCP_OFFER, xid, ip, &self.config);
                        let _ = socket.send_to(&reply, "255.255.255.255:68");
                    }
                }
                DHCP_REQUEST => {
                    let offered_ip = self.allocate_ip(&mac).await;
                    let reply_type = if offered_ip.is_some() { DHCP_ACK } else { DHCP_NAK };
                    let ip = offered_ip.unwrap_or(Ipv4Addr::UNSPECIFIED);
                    let reply = build_reply(reply_type, xid, ip, &self.config);
                    let _ = socket.send_to(&reply, "255.255.255.255:68");

                    if reply_type == DHCP_ACK {
                        self.record_lease(&mac, ip).await;
                    }
                }
                DHCP_INFORM => {
                    // INFORM: client has an IP, just wants options.
                    let ciaddr = Ipv4Addr::new(
                        pkt[BOOTP_CIADDR],
                        pkt[BOOTP_CIADDR + 1],
                        pkt[BOOTP_CIADDR + 2],
                        pkt[BOOTP_CIADDR + 3],
                    );
                    let reply = build_reply(DHCP_ACK, xid, ciaddr, &self.config);
                    let dest = SocketAddr::V4(SocketAddrV4::new(ciaddr, 68));
                    let _ = socket.send_to(&reply, dest);
                }
                _ => {}
            }
        }
    }

    /// Allocate an IP for `mac`, reusing an existing lease if one exists.
    async fn allocate_ip(&self, mac: &str) -> Option<Ipv4Addr> {
        // Check for existing lease.
        let existing = self.leases.find_by_mac(mac).await;
        if let Some(lease) = existing {
            if let Ok(ip) = lease.ip.parse::<Ipv4Addr>() {
                return Some(ip);
            }
        }

        // Find a free IP in range.
        let start = u32::from(self.config.range_start);
        let end = u32::from(self.config.range_end);
        for addr in start..=end {
            let ip = Ipv4Addr::from(addr);
            let ip_str = ip.to_string();
            if !self.leases.is_ip_taken(&ip_str).await {
                return Some(ip);
            }
        }
        None
    }

    async fn record_lease(&self, mac: &str, ip: Ipv4Addr) {
        let expires = chrono::Utc::now()
            + chrono::Duration::from_std(self.config.lease_duration)
                .unwrap_or(chrono::Duration::hours(24));
        let lease = Lease {
            mac: mac.to_owned(),
            ip: ip.to_string(),
            hostname: String::new(),
            expires,
            is_static: false,
        };
        let _ = self.leases.add_or_update(lease).await;
    }
}

/// Build a DHCP broadcast socket with SO_BROADCAST and SO_REUSEADDR.
pub fn build_dhcp_socket(addr: &str) -> Result<StdUdpSocket, crate::DhcpError> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .map_err(crate::DhcpError::Io)?;
    socket.set_reuse_address(true).map_err(crate::DhcpError::Io)?;
    socket.set_broadcast(true).map_err(crate::DhcpError::Io)?;
    let addr: std::net::SocketAddr = addr
        .parse()
        .map_err(|e: std::net::AddrParseError| crate::DhcpError::Config(e.to_string()))?;
    socket.bind(&addr.into()).map_err(crate::DhcpError::Io)?;
    let udp: StdUdpSocket = socket.into();
    udp.set_nonblocking(false).map_err(crate::DhcpError::Io)?;
    Ok(udp)
}

/// Parse DHCP option 53 (Message Type) from the options field.
fn parse_message_type(options: &[u8]) -> Option<u8> {
    let mut i = 0;
    while i < options.len() {
        let code = options[i];
        if code == 255 {
            break; // End option
        }
        if code == 0 {
            i += 1; // Pad option
            continue;
        }
        if i + 1 >= options.len() {
            break;
        }
        let len = options[i + 1] as usize;
        if code == 53 && len == 1 && i + 2 < options.len() {
            return Some(options[i + 2]);
        }
        i += 2 + len;
    }
    None
}

/// Build a minimal DHCP reply packet.
fn build_reply(msg_type: u8, xid: [u8; 4], yiaddr: Ipv4Addr, cfg: &DhcpV4Config) -> Vec<u8> {
    let mut pkt = vec![0u8; BOOTP_OPTIONS + 4 + 64]; // header + magic + options

    pkt[BOOTP_OP] = 2; // BOOTREPLY
    pkt[BOOTP_HTYPE] = 1; // Ethernet
    pkt[BOOTP_HLEN] = 6;
    pkt[BOOTP_XID..BOOTP_XID + 4].copy_from_slice(&xid);
    let yiaddr_bytes = yiaddr.octets();
    pkt[BOOTP_YIADDR..BOOTP_YIADDR + 4].copy_from_slice(&yiaddr_bytes);
    let siaddr_bytes = cfg.gateway.octets();
    pkt[BOOTP_SIADDR..BOOTP_SIADDR + 4].copy_from_slice(&siaddr_bytes);

    // Magic cookie
    pkt[BOOTP_OPTIONS..BOOTP_OPTIONS + 4].copy_from_slice(&DHCP_MAGIC_COOKIE);

    // Options
    let mut opt_idx = BOOTP_OPTIONS + 4;

    // Option 53: DHCP Message Type
    pkt[opt_idx] = 53;
    pkt[opt_idx + 1] = 1;
    pkt[opt_idx + 2] = msg_type;
    opt_idx += 3;

    // Option 1: Subnet Mask
    let mask = cfg.subnet_mask.octets();
    pkt[opt_idx] = 1;
    pkt[opt_idx + 1] = 4;
    pkt[opt_idx + 2..opt_idx + 6].copy_from_slice(&mask);
    opt_idx += 6;

    // Option 3: Router
    let gw = cfg.gateway.octets();
    pkt[opt_idx] = 3;
    pkt[opt_idx + 1] = 4;
    pkt[opt_idx + 2..opt_idx + 6].copy_from_slice(&gw);
    opt_idx += 6;

    // Option 51: Lease Time
    let lease_secs = cfg.lease_duration.as_secs() as u32;
    pkt[opt_idx] = 51;
    pkt[opt_idx + 1] = 4;
    pkt[opt_idx + 2..opt_idx + 6].copy_from_slice(&lease_secs.to_be_bytes());
    opt_idx += 6;

    // Option 255: End
    pkt[opt_idx] = 255;

    pkt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_type_discover() {
        // Option 53, length 1, value 1 (DISCOVER), followed by end option.
        let options = [53u8, 1, 1, 255];
        assert_eq!(parse_message_type(&options), Some(DHCP_DISCOVER));
    }

    #[test]
    fn test_parse_message_type_request() {
        let options = [53u8, 1, 3, 255];
        assert_eq!(parse_message_type(&options), Some(DHCP_REQUEST));
    }

    #[test]
    fn test_parse_message_type_missing() {
        let options = [255u8]; // End option only
        assert_eq!(parse_message_type(&options), None);
    }

    #[test]
    fn test_build_reply_has_magic_cookie() {
        let cfg = DhcpV4Config {
            interface: "eth0".to_owned(),
            gateway: Ipv4Addr::new(192, 168, 1, 1),
            subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
            range_start: Ipv4Addr::new(192, 168, 1, 100),
            range_end: Ipv4Addr::new(192, 168, 1, 200),
            lease_duration: Duration::from_secs(86400),
        };
        let reply = build_reply(DHCP_OFFER, [0, 0, 0, 1], Ipv4Addr::new(192, 168, 1, 100), &cfg);
        assert_eq!(&reply[BOOTP_OPTIONS..BOOTP_OPTIONS + 4], &DHCP_MAGIC_COOKIE);
        assert_eq!(reply[BOOTP_OP], 2); // BOOTREPLY
    }

    #[test]
    fn test_build_reply_message_type_in_options() {
        let cfg = DhcpV4Config {
            interface: "eth0".to_owned(),
            gateway: Ipv4Addr::new(10, 0, 0, 1),
            subnet_mask: Ipv4Addr::new(255, 255, 0, 0),
            range_start: Ipv4Addr::new(10, 0, 0, 10),
            range_end: Ipv4Addr::new(10, 0, 0, 254),
            lease_duration: Duration::from_secs(3600),
        };
        let reply = build_reply(DHCP_ACK, [1, 2, 3, 4], Ipv4Addr::new(10, 0, 0, 10), &cfg);
        // Option 53 starts at BOOTP_OPTIONS + 4
        let opt_start = BOOTP_OPTIONS + 4;
        assert_eq!(reply[opt_start], 53); // option code
        assert_eq!(reply[opt_start + 1], 1); // length
        assert_eq!(reply[opt_start + 2], DHCP_ACK); // value
    }
}

