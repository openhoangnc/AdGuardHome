use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use hickory_proto::op::{Message, MessageType, ResponseCode};
use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};

use crate::DnsError;

/// Trait for handling DNS queries.
#[async_trait]
pub trait QueryHandler: Send + Sync {
    async fn handle(&self, request: &Message) -> Message;
}

/// A simple UDP DNS server.
pub struct DnsServer {
    addr: SocketAddr,
    handler: Arc<dyn QueryHandler>,
}

impl DnsServer {
    /// Create a new DNS server bound to the given address.
    pub fn new(addr: SocketAddr, handler: Arc<dyn QueryHandler>) -> Self {
        Self { addr, handler }
    }

    /// Start the UDP DNS server.
    pub async fn serve_udp(self) -> Result<tokio::task::JoinHandle<()>, DnsError> {
        let socket = Arc::new(tokio::net::UdpSocket::bind(self.addr).await?);
        let handler = self.handler.clone();

        let handle = tokio::spawn(async move {
            let mut buf = vec![0u8; 4096];
            loop {
                let (len, src) = match socket.recv_from(&mut buf).await {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!(error = %e, "UDP recv error");
                        continue;
                    }
                };

                let request = match Message::from_bytes(&buf[..len]) {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::warn!(error = %e, from = %src, "Failed to decode DNS request");
                        continue;
                    }
                };

                let response = handler.handle(&request).await;
                match response.to_bytes() {
                    Ok(bytes) => {
                        if let Err(e) = socket.send_to(&bytes, src).await {
                            tracing::warn!(error = %e, "Failed to send DNS response");
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "Failed to encode DNS response");
                    }
                }
            }
        });

        Ok(handle)
    }
}

/// Build a SERVFAIL response for a given request.
pub fn servfail_response(request: &Message) -> Message {
    let mut response = request.clone();
    response.set_message_type(MessageType::Response);
    response.set_response_code(ResponseCode::ServFail);
    response
}

/// Build an NXDOMAIN response for a given request.
pub fn nxdomain_response(request: &Message) -> Message {
    let mut response = request.clone();
    response.set_message_type(MessageType::Response);
    response.set_response_code(ResponseCode::NXDomain);
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_servfail_response() {
        let mut req = Message::new();
        req.set_message_type(MessageType::Query);
        let resp = servfail_response(&req);
        assert_eq!(resp.response_code(), ResponseCode::ServFail);
    }

    #[test]
    fn test_nxdomain_response() {
        let mut req = Message::new();
        req.set_message_type(MessageType::Query);
        let resp = nxdomain_response(&req);
        assert_eq!(resp.response_code(), ResponseCode::NXDomain);
    }
}
