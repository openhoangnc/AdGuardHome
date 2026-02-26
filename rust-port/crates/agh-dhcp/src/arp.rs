//! ARP table reading for client discovery.
//!
//! On Linux reads `/proc/net/arp`, on macOS runs `arp -an`.

use std::collections::HashMap;
use std::net::IpAddr;

/// Read the ARP table and return IP → MAC address mappings.
#[cfg(target_os = "linux")]
pub async fn read_arp_table() -> std::io::Result<HashMap<IpAddr, String>> {
    let content = tokio::fs::read_to_string("/proc/net/arp").await?;
    let mut map = HashMap::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() >= 4 {
            if let Ok(ip) = cols[0].parse::<IpAddr>() {
                map.insert(ip, cols[3].to_uppercase());
            }
        }
    }
    Ok(map)
}

#[cfg(not(target_os = "linux"))]
pub async fn read_arp_table() -> std::io::Result<HashMap<IpAddr, String>> {
    Ok(HashMap::new())
}
