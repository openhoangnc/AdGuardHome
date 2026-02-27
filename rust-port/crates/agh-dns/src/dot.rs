//! DNS-over-TLS (DoT) — RFC 7858.
//!
//! Listens on a TCP port, wraps each connection with TLS using tokio-rustls,
//! and processes DNS messages length-prefixed per RFC 7858 (2-byte big-endian
//! length before each DNS message).

use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::{DnsError, server::QueryHandler};

/// Serve DNS-over-TLS on `addr` using the provided TLS acceptor and query handler.
pub async fn serve_dot<H>(
    addr: &str,
    tls_acceptor: TlsAcceptor,
    handler: Arc<H>,
) -> Result<(), DnsError>
where
    H: QueryHandler + Send + Sync + 'static,
{
    let listener = TcpListener::bind(addr).await?;
    tracing::info!(addr = %addr, "DoT server listening");

    loop {
        let (stream, peer) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let handler = handler.clone();
        tokio::spawn(async move {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => {
                    if let Err(e) = handle_dot_connection(tls_stream, handler).await {
                        tracing::debug!(peer = %peer, err = %e, "DoT connection error");
                    }
                }
                Err(e) => {
                    tracing::debug!(peer = %peer, err = %e, "DoT TLS handshake failed");
                }
            }
        });
    }
}

/// Handle a single RFC 7858 DNS-over-TLS connection (length-prefixed framing).
pub async fn handle_dot_connection<H, S>(mut stream: S, handler: Arc<H>) -> Result<(), DnsError>
where
    H: QueryHandler + Send + Sync,
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    use hickory_proto::op::Message;
    use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};

    loop {
        // Read 2-byte length prefix.
        let mut len_buf = [0u8; 2];
        match stream.read_exact(&mut len_buf).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(DnsError::Io(e)),
        }
        let msg_len = u16::from_be_bytes(len_buf) as usize;
        if msg_len == 0 {
            break;
        }

        let mut msg_buf = vec![0u8; msg_len];
        stream
            .read_exact(&mut msg_buf)
            .await
            .map_err(DnsError::Io)?;

        let request = match Message::from_bytes(&msg_buf) {
            Ok(m) => m,
            Err(_) => break,
        };
        let response = handler.handle(&request).await;
        let response_bytes = match response.to_bytes() {
            Ok(b) => b,
            Err(_) => break,
        };

        let resp_len = response_bytes.len() as u16;
        stream
            .write_all(&resp_len.to_be_bytes())
            .await
            .map_err(DnsError::Io)?;
        stream
            .write_all(&response_bytes)
            .await
            .map_err(DnsError::Io)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use hickory_proto::op::{Message, MessageType, OpCode, Query, ResponseCode};
    use hickory_proto::rr::{DNSClass, Name, RecordType};
    use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};

    struct EchoHandler;

    #[async_trait]
    impl QueryHandler for EchoHandler {
        async fn handle(&self, req: &Message) -> Message {
            let mut resp = req.clone();
            resp.set_message_type(MessageType::Response);
            resp.set_response_code(ResponseCode::NoError);
            resp
        }
    }

    #[tokio::test]
    async fn test_dot_framing_roundtrip() {
        // Build a minimal DNS query.
        let mut query = Message::new();
        query.set_id(1234);
        query.set_op_code(OpCode::Query);
        query.set_message_type(MessageType::Query);
        let name = Name::from_ascii("example.com.").unwrap();
        let mut q = Query::new();
        q.set_name(name);
        q.set_query_type(RecordType::A);
        q.set_query_class(DNSClass::IN);
        query.add_query(q);
        let query_bytes = query.to_bytes().unwrap();

        // Build the RFC 7858 frame.
        let mut frame: Vec<u8> = (query_bytes.len() as u16).to_be_bytes().to_vec();
        frame.extend_from_slice(&query_bytes);

        // Use an in-memory duplex pipe.
        let (client, server) = tokio::io::duplex(4096);
        let handler = Arc::new(EchoHandler);

        let (mut cr, mut cw) = tokio::io::split(client);

        // Run the server concurrently so write + read can interleave.
        let server_task = tokio::spawn(handle_dot_connection(server, handler));

        // Send the query frame then shut down the write half to signal EOF.
        cw.write_all(&frame).await.unwrap();
        cw.shutdown().await.unwrap();

        // Read the response.
        let mut resp_len_buf = [0u8; 2];
        cr.read_exact(&mut resp_len_buf).await.unwrap();
        let resp_len = u16::from_be_bytes(resp_len_buf) as usize;
        let mut resp_buf = vec![0u8; resp_len];
        cr.read_exact(&mut resp_buf).await.unwrap();

        let response = Message::from_bytes(&resp_buf).unwrap();
        assert_eq!(response.id(), 1234);
        assert_eq!(response.message_type(), MessageType::Response);

        server_task.await.unwrap().unwrap();
    }
}
