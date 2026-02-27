//! API contract tests — authentication endpoints.
//!
//! Tests: POST /control/login, GET /control/logout, POST /control/logout

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

// ── POST /control/login ───────────────────────────────────────────────────────

#[tokio::test]
async fn login_with_no_users_returns_403() {
    let app = common::test_app().await;
    let body = json!({"name": "admin", "password": "secret"});
    let resp = app
        .oneshot(
            Request::post("/control/login")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        resp.status() == StatusCode::FORBIDDEN || resp.status() == StatusCode::UNAUTHORIZED,
        "Expected 403/401, got {}",
        resp.status()
    );
}

#[tokio::test]
async fn login_with_valid_credentials_sets_cookie() {
    use agh_core::config::user::User;

    let dir = tempfile::tempdir().expect("tempdir");
    let cfg_path = dir.path().join("AdGuardHome.yaml");
    let config = std::sync::Arc::new(
        agh_core::config_io::ConfigManager::load(&cfg_path)
            .await
            .expect("load"),
    );
    let hash = bcrypt::hash("password", 10).expect("bcrypt hash");
    config
        .update(|cfg| {
            cfg.users.push(User {
                name: "admin".to_owned(),
                password: hash.clone(),
            });
        })
        .await
        .expect("update");

    let state = agh_web::routes::AppState {
        config,
        sessions: std::sync::Arc::new(agh_web::auth::SessionStore::new()),
    };
    let app = agh_web::create_router(state);

    let body = json!({"name": "admin", "password": "password"});
    let resp = app
        .oneshot(
            Request::post("/control/login")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let set_cookie = resp.headers().get("set-cookie");
    assert!(set_cookie.is_some(), "Expected Set-Cookie header");
    let cookie_val = set_cookie.unwrap().to_str().unwrap();
    assert!(
        cookie_val.contains("agh_session="),
        "Expected agh_session cookie"
    );
}

#[tokio::test]
async fn login_with_wrong_password_returns_403() {
    use agh_core::config::user::User;

    let dir = tempfile::tempdir().expect("tempdir");
    let cfg_path = dir.path().join("AdGuardHome.yaml");
    let config = std::sync::Arc::new(
        agh_core::config_io::ConfigManager::load(&cfg_path)
            .await
            .expect("load"),
    );
    let hash = bcrypt::hash("correct_password", 4).expect("bcrypt hash");
    config
        .update(|cfg| {
            cfg.users.push(User {
                name: "admin".to_owned(),
                password: hash,
            });
        })
        .await
        .expect("update");

    let state = agh_web::routes::AppState {
        config,
        sessions: std::sync::Arc::new(agh_web::auth::SessionStore::new()),
    };
    let app = agh_web::create_router(state);

    let body = json!({"name": "admin", "password": "wrong_password"});
    let resp = app
        .oneshot(
            Request::post("/control/login")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ── GET /control/logout ───────────────────────────────────────────────────────

#[tokio::test]
async fn logout_clears_session_cookie() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(Request::get("/control/logout").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let set_cookie = resp
        .headers()
        .get("set-cookie")
        .map(|v| v.to_str().unwrap_or(""));
    assert!(
        set_cookie.map(|c| c.contains("Max-Age=0")).unwrap_or(false),
        "logout must expire the session cookie"
    );
}

// ── Content-Type checks ───────────────────────────────────────────────────────

#[tokio::test]
async fn login_without_json_content_type_returns_error() {
    let app = common::test_app().await;
    let resp = app
        .oneshot(
            Request::post("/control/login")
                .body(Body::from(r#"{"name":"admin","password":"x"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    // axum returns 415 or 422 without the correct Content-Type.
    assert!(
        resp.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE
            || resp.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "got {}",
        resp.status()
    );
}
