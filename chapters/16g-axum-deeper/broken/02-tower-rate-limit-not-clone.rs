//! tower's `RateLimit` is not `Clone`, and axum clones services for every request
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: the trait `Clone` is not implemented for `HandleError<RateLimit<Route>, ...>`
//! Fix: a per-client limiter that is built for axum — `tower_governor`, see src/limits.rs.
// expected-error: the trait `Clone` is not implemented for `HandleError<RateLimit<Route>
// check-with: cargo

use std::time::Duration;

use axum::{BoxError, Router, error_handling::HandleErrorLayer, http::StatusCode, routing::get};
use tower::ServiceBuilder;

async fn too_many(_: BoxError) -> StatusCode {
    StatusCode::TOO_MANY_REQUESTS
}

fn main() {
    let _app: Router = Router::new().route("/login", get(|| async { "login" })).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(too_many))
            .rate_limit(5, Duration::from_secs(60)),
    );
}
