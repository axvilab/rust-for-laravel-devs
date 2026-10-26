//! Custom middleware. Everything generic (trace, timeout, CORS, compression)
//! comes from `tower-http`; only the app-specific bits live here.

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tracing::Span;

pub const REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Takes the incoming `x-request-id` (from a proxy) or generates one, puts it
/// on the request (so `TraceLayer` can log it) and on the response.
///
/// Laravel equivalent: `handle($request, Closure $next)`; `next.run(req).await`
/// is `$next($request)`.
pub async fn request_id(mut req: Request, next: Next) -> Response {
    let id = req
        .headers()
        .get(&REQUEST_ID)
        .filter(|v| !v.is_empty() && v.len() <= 128)
        .cloned()
        .unwrap_or_else(|| {
            HeaderValue::from_str(&uuid::Uuid::new_v4().to_string())
                .expect("a UUID is always a valid header value")
        });
    req.headers_mut().insert(REQUEST_ID, id.clone());
    let mut res = next.run(req).await;
    res.headers_mut().insert(REQUEST_ID, id);
    res
}

/// Span for `TraceLayer`: method, path and request id on every log line.
pub fn make_span(req: &Request) -> Span {
    let request_id = req
        .headers()
        .get(&REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-");
    tracing::info_span!(
        "http",
        method = %req.method(),
        path = %req.uri().path(),
        request_id = %request_id,
    )
}

/// Empty list -> permissive (dev). In production set `CORS_ORIGINS`.
pub fn cors(origins: &[String]) -> CorsLayer {
    if origins.is_empty() {
        return CorsLayer::permissive();
    }
    let list: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(list))
        .allow_methods(Any)
        .allow_headers(Any)
}
