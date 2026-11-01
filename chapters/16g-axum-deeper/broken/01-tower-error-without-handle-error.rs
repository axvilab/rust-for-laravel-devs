//! tower's own `TimeoutLayer` straight into `Router::layer`: it returns errors, axum wants `Infallible`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: the trait bound `Infallible: From<Box<(dyn std::error::Error + Send + Sync + 'static)>>` is not satisfied
//! Fix: put `HandleErrorLayer` in front of it — see src/limits.rs (`overload_router`),
//! or use tower-http's `TimeoutLayer`, which answers with a status code itself.
// expected-error: the trait bound `Infallible: From<Box<(dyn std::error::Error + Send + Sync + 'static)>>` is not satisfied
// check-with: cargo

use std::time::Duration;

use axum::{Router, routing::get};

fn main() {
    let _app: Router = Router::new()
        .route("/report", get(|| async { "report" }))
        .layer(tower::timeout::TimeoutLayer::new(Duration::from_secs(1)));
}
