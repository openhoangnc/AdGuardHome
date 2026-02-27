use std::sync::Arc;

use axum::extract::{Json, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post, put};
use serde::{Deserialize, Serialize};
use serde_json::json;

use agh_core::config::tls::TlsConfig;
use agh_core::config_io::ConfigManager;

use crate::auth::{extract_session_token, make_session_cookie, SessionStore};
use crate::tls_config::validate_cert;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ConfigManager>,
    pub sessions: Arc<SessionStore>,
}

/// Build the full axum router.
pub fn create_router(state: AppState) -> Router {
    let api = Router::new()
        // Auth routes (no session required)
        .route("/control/login", post(login_handler))
        .route("/control/logout", get(logout_handler))
        .route("/control/logout", post(logout_handler))
        // Status (requires auth)
        .route("/control/status", get(status_handler))
        // DNS info
        .route("/control/dns_info", get(dns_info_handler))
        // Filtering
        .route("/control/filtering/status", get(filtering_status_handler))
        // Setup wizard
        .route("/install/check_config", post(install_check_config_handler))
        .route("/install/configure", post(install_configure_handler))
        .route("/install/get_addresses", get(install_get_addresses_handler))
        // Version
        .route("/control/version.json", get(version_handler))
        // TLS
        .route("/control/tls/status", get(tls_status_handler))
        .route("/control/tls/configure", post(tls_configure_handler))
        .route("/control/tls/validate", post(tls_validate_handler))
        .with_state(state.clone());

    // Frontend catch-all
    Router::new()
        .merge(api)
        .fallback(crate::frontend::serve_frontend)
}

// ── Request / response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct StatusResponse {
    #[serde(rename = "dhcp_available")]
    dhcp_available: bool,
    #[serde(rename = "dns_addresses")]
    dns_addresses: Vec<String>,
    #[serde(rename = "dns_port")]
    dns_port: u16,
    #[serde(rename = "http_port")]
    http_port: u16,
    #[serde(rename = "is_running")]
    is_running: bool,
    #[serde(rename = "language")]
    language: String,
    #[serde(rename = "running")]
    running: bool,
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "welcome_greeting")]
    welcome_greeting: bool,
}

// ── Route handlers ────────────────────────────────────────────────────────────

async fn login_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let cfg = state.config.get_async().await;
    let user = cfg.users.iter().find(|u| u.name == body.name);

    let ok = match user {
        Some(u) => crate::auth::verify_password(&u.password, &body.password),
        None => false,
    };

    if ok {
        let token = state.sessions.create(&body.name);
        let cookie = make_session_cookie(&token, false);
        (
            StatusCode::OK,
            [(header::SET_COOKIE, cookie)],
            Json(json!({})),
        )
            .into_response()
    } else {
        (StatusCode::FORBIDDEN, Json(json!({"message": "invalid credentials"}))).into_response()
    }
}

async fn logout_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(token) = extract_session_token(&_headers) {
        state.sessions.remove(&token);
    }
    let clear_cookie = "agh_session=; Max-Age=0; Path=/".to_string();
    (StatusCode::OK, [(header::SET_COOKIE, clear_cookie)], Json(json!({}))).into_response()
}

async fn status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let cfg = state.config.get_async().await;
    let http_port: u16 = cfg.http.address
        .rsplit(':')
        .next()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    Json(StatusResponse {
        dhcp_available: false,
        dns_addresses: cfg.dns.bind_hosts.clone(),
        dns_port: cfg.dns.port,
        http_port,
        is_running: true,
        language: "en".to_owned(),
        running: true,
        version: env!("CARGO_PKG_VERSION").to_owned(),
        welcome_greeting: cfg.users.is_empty(),
    })
}

async fn dns_info_handler(State(state): State<AppState>) -> impl IntoResponse {
    let cfg = state.config.get_async().await;
    Json(json!({
        "upstream_dns": cfg.dns.upstream_dns,
        "bootstrap_dns": cfg.dns.bootstrap_dns,
        "protection_enabled": cfg.dns.filtering_enabled,
        "ratelimit": 0,
        "blocking_mode": "default",
        "blocking_ipv4": "",
        "blocking_ipv6": "",
        "edns_cs_enabled": cfg.dns.edns_client_subnet.enabled,
        "dnssec_enabled": cfg.dns.enable_dnssec,
        "disable_ipv6": cfg.dns.aaaa_disabled,
        "upstream_mode": cfg.dns.upstream_mode,
        "cache_size": cfg.dns.cache_size,
        "cache_ttl_min": cfg.dns.cache_ttl_min,
        "cache_ttl_max": cfg.dns.cache_ttl_max,
        "cache_optimistic": cfg.dns.cache_optimistic,
        "resolve_clients": cfg.dns.resolve_clients,
        "local_ptr_upstreams": cfg.dns.local_ptr_upstreams,
        "use_private_ptr_resolvers": cfg.dns.use_private_ptr_resolvers,
        "default_local_ptr_upstreams": [],
    }))
}

async fn filtering_status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let cfg = state.config.get_async().await;
    Json(json!({
        "enabled": cfg.dns.filtering_enabled,
        "interval": cfg.dns.filters_update_interval,
        "filters": cfg.filters,
        "whitelist_filters": cfg.whitelist_filters,
        "user_rules": cfg.user_rules,
    }))
}

async fn install_get_addresses_handler() -> impl IntoResponse {
    Json(json!({
        "web_port": 3000,
        "dns_port": 53,
        "interfaces": {}
    }))
}

async fn install_check_config_handler(
    Json(_body): Json<serde_json::Value>,
) -> impl IntoResponse {
    Json(json!({"dns": {"status": "", "can_autofix": false}, "web": {"status": "", "can_autofix": false}}))
}

async fn install_configure_handler(
    State(_state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> impl IntoResponse {
    // In real implementation, this saves initial config.
    StatusCode::OK
}

async fn version_handler() -> impl IntoResponse {
    Json(json!({
        "disabled": false,
        "new_version": "",
        "announcement": "",
        "announcement_url": "",
        "can_autoupdate": false,
    }))
}

// ── TLS handlers ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TlsConfigRequest {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    server_name: String,
    #[serde(default)]
    force_https: bool,
    #[serde(default = "default_port_https")]
    port_https: u16,
    #[serde(default = "default_port_dot")]
    port_dns_over_tls: u16,
    #[serde(default = "default_port_doq")]
    port_dns_over_quic: u16,
    #[serde(default)]
    certificate_chain: String,
    #[serde(default)]
    private_key: String,
}

fn default_port_https() -> u16 { 443 }
fn default_port_dot() -> u16 { 853 }
fn default_port_doq() -> u16 { 784 }

fn tls_status_json(tls: &TlsConfig) -> serde_json::Value {
    let info = validate_cert(&tls.certificate_chain, &tls.private_key);
    json!({
        "enabled": tls.enabled,
        "server_name": tls.server_name,
        "force_https": tls.force_https,
        "port_https": tls.port_https,
        "port_dns_over_tls": tls.port_dns_over_tls,
        "port_dns_over_quic": tls.port_dns_over_quic,
        "certificate_chain": tls.certificate_chain,
        "private_key": tls.private_key,
        "valid_cert": info.is_valid,
        "valid_key": info.valid_key,
        "valid_pair": info.valid_pair,
        "not_after": info.not_after,
        "warning_validation": info.warning,
    })
}

async fn tls_status_handler(State(state): State<AppState>) -> impl IntoResponse {
    let cfg = state.config.get_async().await;
    Json(tls_status_json(&cfg.tls))
}

async fn tls_configure_handler(
    State(state): State<AppState>,
    Json(body): Json<TlsConfigRequest>,
) -> impl IntoResponse {
    state
        .config
        .update(|cfg| {
            cfg.tls.enabled = body.enabled;
            cfg.tls.server_name = body.server_name.clone();
            cfg.tls.force_https = body.force_https;
            cfg.tls.port_https = body.port_https;
            cfg.tls.port_dns_over_tls = body.port_dns_over_tls;
            cfg.tls.port_dns_over_quic = body.port_dns_over_quic;
            cfg.tls.certificate_chain = body.certificate_chain.clone();
            cfg.tls.private_key = body.private_key.clone();
        })
        .await
        .map(|_| {
            let info = validate_cert(&body.certificate_chain, &body.private_key);
            (StatusCode::OK, Json(json!({
                "valid_cert": info.is_valid,
                "valid_key": info.valid_key,
                "valid_pair": info.valid_pair,
                "warning_validation": info.warning,
            }))).into_response()
        })
        .unwrap_or_else(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": e.to_string()}))).into_response()
        })
}

async fn tls_validate_handler(
    Json(body): Json<TlsConfigRequest>,
) -> impl IntoResponse {
    let info = validate_cert(&body.certificate_chain, &body.private_key);
    Json(json!({
        "enabled": body.enabled,
        "server_name": body.server_name,
        "valid_cert": info.is_valid,
        "valid_key": info.valid_key,
        "valid_pair": info.valid_pair,
        "not_after": info.not_after,
        "warning_validation": info.warning,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    async fn test_state() -> AppState {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg_path = dir.path().join("AdGuardHome.yaml");
        let config = Arc::new(
            agh_core::config_io::ConfigManager::load(&cfg_path).await.expect("load")
        );
        AppState {
            config,
            sessions: Arc::new(SessionStore::new()),
        }
    }

    #[tokio::test]
    async fn test_status_returns_200() {
        let state = test_state().await;
        let app = create_router(state);
        let response = app
            .oneshot(Request::get("/control/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_dns_info_returns_200() {
        let state = test_state().await;
        let app = create_router(state);
        let response = app
            .oneshot(Request::get("/control/dns_info").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_version_returns_200() {
        let state = test_state().await;
        let app = create_router(state);
        let response = app
            .oneshot(Request::get("/control/version.json").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_tls_status_returns_200() {
        let state = test_state().await;
        let app = create_router(state);
        let response = app
            .oneshot(Request::get("/control/tls/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_tls_validate_empty_certs() {
        let state = test_state().await;
        let app = create_router(state);
        let body = serde_json::json!({
            "enabled": false,
            "certificate_chain": "",
            "private_key": ""
        });
        let response = app
            .oneshot(
                Request::post("/control/tls/validate")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["valid_pair"], false);
    }
}
