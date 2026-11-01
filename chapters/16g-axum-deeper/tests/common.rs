// Shared helpers for the integration tests (each tests/*.rs includes this via `mod common;`).
#![allow(dead_code)]

use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

pub struct Res {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

/// One request through the router, without a TCP socket.
pub async fn send(app: Router, req: Request<Body>) -> Res {
    let res = app.oneshot(req).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    Res {
        status,
        headers,
        body: String::from_utf8_lossy(&bytes).into_owned(),
    }
}

pub fn get(uri: &str) -> Request<Body> {
    Request::get(uri).body(Body::empty()).unwrap()
}
