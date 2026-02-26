mod mod;

use std::sync::Arc;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use agh_web::auth::SessionStore;
use agh_web::routes::AppState;

async fn test_app() -> axum::Router {
    let config = super::mod::test_config().await;
    let state = AppState { config, sessions: Arc::new(SessionStore::new()) };
    agh_web::create_router(state)
}

#[tokio::test]
#[ignore = "requires running server (TASK-40)"]
async fn test_status_endpoint_returns_200() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/control/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("running").is_some());
    assert!(json.get("dns_port").is_some());
}
