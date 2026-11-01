//! tower's own middleware (they return errors) behind `HandleErrorLayer`,
//! and per-IP rate limiting with `tower_governor`.

use std::time::Duration;

use axum::{
    BoxError, Router,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use tower::{ServiceBuilder, load_shed::error::Overloaded, timeout::error::Elapsed};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

/// What the tower layers below return as `Err` becomes a normal HTTP response here.
async fn handle_tower_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "took too long".into())
    } else if err.is::<Overloaded>() {
        (StatusCode::SERVICE_UNAVAILABLE, "busy, try again".into())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unhandled: {err}"),
        )
    }
}

async fn report() -> &'static str {
    tokio::time::sleep(Duration::from_millis(300)).await;
    "report ready"
}

/// At most 2 reports at a time; the third gets 503 at once instead of queueing.
pub fn overload_router() -> Router {
    Router::new().route("/report", get(report)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_tower_error))
            .load_shed()
            .concurrency_limit(2)
            .timeout(Duration::from_secs(1)),
    )
}

/// 3 requests in a burst, then one more every 2 seconds — per client IP.
pub fn rate_limited_router() -> Router {
    let config = GovernorConfigBuilder::default()
        .per_second(2) // NOT "2 per second": one new request every 2 seconds
        .burst_size(3)
        .use_headers() // x-ratelimit-* in responses
        .finish()
        .expect("period and burst are not zero");

    Router::new()
        .route("/login", get(|| async { "login form" }))
        .layer(GovernorLayer::new(config))
}

/// The same limiter with a JSON error instead of the plain-text default.
pub fn rate_limited_json() -> Router {
    let config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(3)
        .finish()
        .expect("period and burst are not zero");

    let layer = GovernorLayer::new(config).error_handler(|err| {
        let (status, msg) = match err {
            tower_governor::GovernorError::TooManyRequests { wait_time, .. } => (
                StatusCode::TOO_MANY_REQUESTS,
                format!("too many requests, retry in {wait_time}s"),
            ),
            other => (StatusCode::INTERNAL_SERVER_ERROR, other.to_string()),
        };
        json_error(status, &msg)
    });

    Router::new()
        .route("/login", get(|| async { "login form" }))
        .layer(layer)
}

fn json_error(status: StatusCode, msg: &str) -> Response {
    (status, axum::Json(serde_json::json!({ "error": msg }))).into_response()
}
