//! Integration test helpers shared across API contract tests.

use std::sync::Arc;

use agh_web::routes::AppState;
use agh_web::auth::SessionStore;
use axum::Router;

pub async fn test_app() -> Router {
    // Install rustls ring provider once per test process (no-op if already installed).
    let _ = rustls::crypto::ring::default_provider().install_default();
    let dir = tempfile::tempdir().expect("tempdir");
    let cfg_path = dir.path().join("AdGuardHome.yaml");
    // Leak so tempdir survives the test.
    std::mem::forget(dir);
    let config = Arc::new(
        agh_core::config_io::ConfigManager::load(&cfg_path)
            .await
            .expect("load config"),
    );
    let state = AppState { config, sessions: Arc::new(SessionStore::new()) };
    agh_web::create_router(state)
}

/// Convenience: read an axum response body to bytes.
pub async fn body_bytes(body: axum::body::Body) -> bytes::Bytes {
    axum::body::to_bytes(body, usize::MAX).await.expect("body")
}

/// Convenience: parse axum response body to JSON value.
pub async fn body_json(body: axum::body::Body) -> serde_json::Value {
    let b = body_bytes(body).await;
    serde_json::from_slice(&b).expect("JSON")
}
