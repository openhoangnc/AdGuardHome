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
async fn test_login_no_users_returns_forbidden() {
    let app = test_app().await;
    let body = serde_json::json!({"name": "admin", "password": "wrong"});
    let response = app
        .oneshot(
            Request::post("/control/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    // No users configured → forbidden
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires running server (TASK-40)"]
async fn test_logout_clears_cookie() {
    let app = test_app().await;
    let response = app
        .oneshot(Request::get("/control/logout").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let set_cookie = response.headers().get("set-cookie");
    assert!(set_cookie.is_some());
}
