use std::collections::HashMap;

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use hickory_proto::op::Message;
use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};

use crate::server::QueryHandler;

/// Handler for `POST /dns-query` (RFC 8484 DoH).
pub async fn doh_post_handler(
    State(handler): State<std::sync::Arc<dyn QueryHandler>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Validate content-type.
    if let Some(ct) = headers.get("content-type") {
        if ct.as_bytes() != b"application/dns-message" {
            return (StatusCode::UNSUPPORTED_MEDIA_TYPE, Bytes::new());
        }
    }

    match process_dns_message(&handler, &body).await {
        Ok(bytes) => (StatusCode::OK, bytes),
        Err(_) => (StatusCode::BAD_REQUEST, Bytes::new()),
    }
}

/// Handler for `GET /dns-query?dns=<base64url>` (RFC 8484 DoH).
pub async fn doh_get_handler(
    State(handler): State<std::sync::Arc<dyn QueryHandler>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let dns_param = match params.get("dns") {
        Some(v) => v,
        None => return (StatusCode::BAD_REQUEST, Bytes::new()),
    };

    let decoded = match base64url_decode(dns_param) {
        Ok(b) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, Bytes::new()),
    };

    match process_dns_message(&handler, &decoded).await {
        Ok(bytes) => (StatusCode::OK, bytes),
        Err(_) => (StatusCode::BAD_REQUEST, Bytes::new()),
    }
}

async fn process_dns_message(
    handler: &std::sync::Arc<dyn QueryHandler>,
    bytes: &[u8],
) -> Result<Bytes, ()> {
    let request = Message::from_bytes(bytes).map_err(|_| ())?;
    let response = handler.handle(&request).await;
    let encoded = response.to_bytes().map_err(|_| ())?;
    Ok(Bytes::from(encoded))
}

fn base64url_decode(input: &str) -> Result<Vec<u8>, ()> {
    // Simple base64url decoder (no padding required per RFC 8484).
    let padded = match input.len() % 4 {
        0 => input.to_owned(),
        2 => format!("{input}=="),
        3 => format!("{input}="),
        _ => return Err(()),
    };
    let standard = padded.replace('-', "+").replace('_', "/");
    // Use a simple approach: replace chars and decode
    let bytes = base64_decode_std(&standard)?;
    Ok(bytes)
}

fn base64_decode_std(input: &str) -> Result<Vec<u8>, ()> {
    // Manual base64 decode for no-extra-dep requirement.
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let input = input.trim_end_matches('=');
    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u32;

    for &b in input.as_bytes() {
        let val = CHARS.iter().position(|&c| c == b).ok_or(())? as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(((buf >> bits) & 0xff) as u8);
        }
    }
    Ok(output)
}
