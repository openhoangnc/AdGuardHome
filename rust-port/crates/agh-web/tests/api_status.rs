//! API contract tests — status, dns_info, and version endpoints.

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

// ── GET /control/status ───────────────────────────────────────────────────────

#[tokio::test]
async fn status_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(Request::get("/control/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn status_returns_json_with_required_fields() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(Request::get("/control/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    // Required fields per OpenAPI spec.
    assert!(body.get("running").is_some(), "missing 'running'");
    assert!(body.get("dns_port").is_some(), "missing 'dns_port'");
    assert!(body.get("http_port").is_some(), "missing 'http_port'");
    assert!(
        body.get("protection_enabled").is_some(),
        "missing 'protection_enabled'"
    );
    assert!(body.get("version").is_some(), "missing 'version'");
}

#[tokio::test]
async fn status_content_type_is_json() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(Request::get("/control/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let ct = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.contains("application/json"),
        "Content-Type should be JSON, got: {ct}"
    );
}

// ── GET /control/dns_info ─────────────────────────────────────────────────────

#[tokio::test]
async fn dns_info_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/dns_info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn dns_info_returns_required_fields() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/dns_info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    assert!(body.get("upstream_dns").is_some(), "missing 'upstream_dns'");
    assert!(
        body.get("filtering_enabled").is_some(),
        "missing 'filtering_enabled'"
    );
    assert!(
        body.get("protection_enabled").is_some(),
        "missing 'protection_enabled'"
    );
}

// ── GET /control/version.json ─────────────────────────────────────────────────

#[tokio::test]
async fn version_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/version.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn version_returns_required_fields() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/version.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    assert!(body.get("disabled").is_some(), "missing 'disabled'");
    assert!(body.get("new_version").is_some(), "missing 'new_version'");
}

// ── GET /control/filtering/status ────────────────────────────────────────────

#[tokio::test]
async fn filtering_status_returns_200() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/filtering/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn filtering_status_returns_filters_array() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::get("/control/filtering/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = common::body_json(resp.into_body()).await;
    assert!(body.get("enabled").is_some(), "missing 'enabled'");
    assert!(body.get("filters").is_some(), "missing 'filters'");
}
