//! API contract tests — setup wizard (install) endpoints.
//!
//! Tests: GET /install/get_addresses,
//!        POST /install/check_config,
//!        POST /install/configure

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

// ── GET /install/get_addresses ────────────────────────────────────────────────

#[tokio::test]
async fn install_get_addresses_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/install/get_addresses")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn install_get_addresses_returns_interfaces() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/install/get_addresses")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    // Spec requires 'interfaces' and 'web_port' fields.
    assert!(body.get("interfaces").is_some(), "missing 'interfaces'");
    assert!(body.get("web_port").is_some(), "missing 'web_port'");
    assert!(body.get("dns_port").is_some(), "missing 'dns_port'");
}

// ── POST /install/check_config ────────────────────────────────────────────────

#[tokio::test]
async fn install_check_config_returns_200() {
    let app = common::test_app().await;
    let body = json!({
        "web": {"ip": "0.0.0.0", "port": 3000, "autofix": false},
        "dns": {"ip": "0.0.0.0", "port": 53, "autofix": false}
    });
    let resp = app
        .oneshot(
            Request::post("/install/check_config")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn install_check_config_returns_status_fields() {
    let app = common::test_app().await;
    let body = json!({
        "web": {"ip": "0.0.0.0", "port": 3000, "autofix": false},
        "dns": {"ip": "0.0.0.0", "port": 53, "autofix": false}
    });
    let resp = app
        .oneshot(
            Request::post("/install/check_config")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let json = common::body_json(resp.into_body()).await;
    assert!(json.get("web").is_some(), "missing 'web'");
    assert!(json.get("dns").is_some(), "missing 'dns'");
}

// ── POST /install/configure ───────────────────────────────────────────────────

#[tokio::test]
async fn install_configure_returns_200() {
    let app = common::test_app().await;
    let body = json!({
        "web": {"ip": "0.0.0.0", "port": 3000},
        "dns": {"ip": "0.0.0.0", "port": 53},
        "username": "admin",
        "password": "admin123"
    });
    let resp = app
        .oneshot(
            Request::post("/install/configure")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
