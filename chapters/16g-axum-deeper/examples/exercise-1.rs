//! Exercise 1: SetRequestIdLayer above vs below TraceLayer — whose span gets the id
// Runs one request through each order and prints the log lines. No server needed.

use std::time::Duration;

use axum::{
    Router,
    body::Body,
    http::{HeaderName, Request},
    routing::get,
};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;

const REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

fn span(req: &Request<Body>) -> tracing::Span {
    let id = req.headers().get(&REQUEST_ID).and_then(|v| v.to_str().ok());
    tracing::info_span!("http", id = id.unwrap_or("-"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_ansi(false)
        .init();
    let trace = || {
        TraceLayer::new_for_http()
            .make_span_with(span)
            .on_response(DefaultOnResponse::new().level(Level::INFO))
    };

    let right = Router::new().route("/", get(|| async { "ok" })).layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(REQUEST_ID, MakeRequestUuid)) // outer: id first
            .layer(trace()),
    );
    let wrong = Router::new().route("/", get(|| async { "ok" })).layer(
        ServiceBuilder::new()
            .layer(trace()) // outer: logs before the id exists
            .layer(SetRequestIdLayer::new(REQUEST_ID, MakeRequestUuid)),
    );

    println!("SetRequestId outside TraceLayer:");
    right
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    println!("SetRequestId inside TraceLayer:");
    wrong
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(10)).await;
}
