//! Static files and an SPA: `ServeDir`, `ServeFile`, `fallback_service`, cache headers.

use std::path::PathBuf;

use axum::{
    Json, Router,
    http::{HeaderValue, StatusCode, header},
    response::Response,
    routing::get,
};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

/// Where the built front end lives. `ServeDir::new("dist")` would be relative to the
/// CURRENT directory of the process, not to this crate, hence the absolute path.
pub fn dist_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist")
}

const IMMUTABLE: &str = "public, max-age=31536000, immutable";

fn api() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        // without its own fallback an unknown /api/... would get index.html with 200
        .fallback(|| async {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "not found" })),
            )
        })
}

pub fn app() -> Router {
    let dist = dist_dir();

    // /assets/app-3f9a1c.js: the hash changes with every build -> cache "forever"
    let assets = Router::new()
        .fallback_service(ServeDir::new(dist.join("assets")))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            |res: &Response| {
                // only for real files: a cached-forever 404 would outlive the next deploy
                res.status()
                    .is_success()
                    .then(|| HeaderValue::from_static(IMMUTABLE))
            },
        ));

    // everything else: the file if it exists, otherwise index.html (the SPA router decides)
    let spa = ServeDir::new(&dist).fallback(ServeFile::new(dist.join("index.html")));

    Router::new()
        .nest("/api", api())
        .nest("/assets", assets)
        .fallback_service(spa)
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"), // index.html: always ask the server
        ))
}

/// For comparison: `not_found_service` serves index.html too, but keeps the 404 status.
pub fn app_with_404() -> Router {
    let dist = dist_dir();
    Router::new().fallback_service(
        ServeDir::new(&dist).not_found_service(ServeFile::new(dist.join("index.html"))),
    )
}
