//! Frontend SPA embedding and serving.
//!
//! The prebuilt frontend (`build/` directory) is embedded at compile time
//! using `rust-embed`. Falls back to a minimal response if the build
//! directory does not exist (development mode without frontend build).

use axum::body::Body;
use axum::http::{header, Response, StatusCode, Uri};
use axum::response::IntoResponse;

// Embed the frontend build directory if it exists.
// If `build/` doesn't exist at compile time, RustEmbed serves nothing.
#[derive(rust_embed::RustEmbed)]
#[folder = "../../../build"]
#[prefix = "/"]
struct FrontendAssets;

/// Serve the frontend SPA. Falls back to index.html for unknown paths.
pub async fn serve_frontend(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    // Try exact match first.
    if let Some(asset) = FrontendAssets::get(path) {
        let mime = mime_type(path);
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime)
            .body(Body::from(asset.data.to_vec()))
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    // Fall back to index.html for SPA routing.
    if let Some(index) = FrontendAssets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(index.data.to_vec()))
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    // No frontend assets embedded (development mode).
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Frontend not built. Run `make js` first."))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

fn mime_type(path: &str) -> &'static str {
    if path.ends_with(".html") { "text/html; charset=utf-8" }
    else if path.ends_with(".js") { "application/javascript" }
    else if path.ends_with(".css") { "text/css" }
    else if path.ends_with(".json") { "application/json" }
    else if path.ends_with(".png") { "image/png" }
    else if path.ends_with(".svg") { "image/svg+xml" }
    else if path.ends_with(".ico") { "image/x-icon" }
    else { "application/octet-stream" }
}
