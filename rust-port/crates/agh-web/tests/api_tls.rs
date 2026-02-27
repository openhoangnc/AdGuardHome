//! API contract tests — TLS endpoints.
//!
//! Tests: GET /control/tls/status, POST /control/tls/validate, POST /control/tls/configure

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

// ── GET /control/tls/status ───────────────────────────────────────────────────

#[tokio::test]
async fn tls_status_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/tls/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn tls_status_returns_required_fields() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/tls/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    for field in &[
        "enabled",
        "server_name",
        "port_https",
        "port_dns_over_tls",
        "valid_cert",
        "valid_key",
        "valid_pair",
        "warning_validation",
    ] {
        assert!(body.get(field).is_some(), "missing '{field}'");
    }
}

#[tokio::test]
async fn tls_status_disabled_by_default() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/tls/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    assert_eq!(body["enabled"], false, "TLS should be disabled by default");
}

// ── POST /control/tls/validate ───────────────────────────────────────────────

#[tokio::test]
async fn tls_validate_empty_certs_returns_invalid() {
    let app = common::test_app().await;
    let body = json!({"enabled": false, "certificate_chain": "", "private_key": ""});
    let resp = app
        .oneshot(
            Request::post("/control/tls/validate")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = common::body_json(resp.into_body()).await;
    assert_eq!(json["valid_cert"], false);
    assert_eq!(json["valid_key"], false);
    assert_eq!(json["valid_pair"], false);
}

#[tokio::test]
async fn tls_validate_real_self_signed_cert_valid() {
    // Generate a self-signed cert using rcgen.
    let mut params = rcgen::CertificateParams::new(vec!["localhost".to_owned()]).unwrap();
    let key = rcgen::KeyPair::generate().unwrap();
    let cert = params.self_signed(&key).unwrap();
    let cert_pem = cert.pem();
    let key_pem = key.serialize_pem();

    let app = common::test_app().await;
    let body = json!({
        "enabled": true,
        "server_name": "localhost",
        "certificate_chain": cert_pem,
        "private_key": key_pem,
    });
    let resp = app
        .oneshot(
            Request::post("/control/tls/validate")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = common::body_json(resp.into_body()).await;
    assert_eq!(json["valid_key"], true, "key should be valid");
    assert_eq!(json["valid_pair"], true, "cert+key pair should be valid");
}

#[tokio::test]
async fn tls_validate_mismatched_key_returns_invalid_pair() {
    let mut params1 = rcgen::CertificateParams::new(vec!["host1.test".to_owned()]).unwrap();
    let key1 = rcgen::KeyPair::generate().unwrap();
    let cert1 = params1.self_signed(&key1).unwrap();

    let key2 = rcgen::KeyPair::generate().unwrap();

    let app = common::test_app().await;
    let body = json!({
        "certificate_chain": cert1.pem(),
        "private_key": key2.serialize_pem(),
    });
    let resp = app
        .oneshot(
            Request::post("/control/tls/validate")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = common::body_json(resp.into_body()).await;
    // Key is valid on its own, but the pair doesn't match.
    assert_eq!(json["valid_key"], true, "key itself is valid");
    assert_eq!(
        json["valid_pair"], false,
        "mismatched pair should be invalid"
    );
}

// ── POST /control/tls/configure ──────────────────────────────────────────────

#[tokio::test]
async fn tls_configure_updates_config() {
    let app = common::test_app().await;
    let body = json!({
        "enabled": false,
        "server_name": "test.example.com",
        "port_https": 8443,
        "certificate_chain": "",
        "private_key": "",
    });
    let resp = app
        .oneshot(
            Request::post("/control/tls/configure")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
