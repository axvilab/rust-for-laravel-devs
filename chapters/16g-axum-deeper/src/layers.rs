//! Layers: the order of `.layer()`, a hand-written tower `Service`, the tower-http stack,
//! `route_layer` vs `layer`.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Request, State},
    http::{HeaderName, HeaderValue, Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower::{Layer, Service, ServiceBuilder};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// ---------------------------------------------------------------------------
// 1. The order of `.layer()`
// ---------------------------------------------------------------------------

/// Prints on the way in and on the way out, and appends its name to `x-layers`
/// in the response, so a test can see the order too.
pub async fn step(State(name): State<&'static str>, req: Request, next: Next) -> Response {
    println!("-> {name}");
    let mut res = next.run(req).await;
    println!("<- {name}");
    res.headers_mut()
        .append("x-layers", HeaderValue::from_static(name));
    res
}

async fn handler() -> &'static str {
    println!("   handler");
    "ok"
}

/// `.layer()` one by one: the LAST one added is the OUTERMOST.
pub fn order_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn_with_state("A", step))
        .layer(middleware::from_fn_with_state("B", step))
        .layer(middleware::from_fn_with_state("C", step))
}

/// `ServiceBuilder`: top to bottom = outside to inside, the way you read it.
pub fn order_builder() -> Router {
    Router::new().route("/", get(handler)).layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn_with_state("A", step))
            .layer(middleware::from_fn_with_state("B", step))
            .layer(middleware::from_fn_with_state("C", step)),
    )
}

// ---------------------------------------------------------------------------
// 2. A Layer and a Service written by hand
// ---------------------------------------------------------------------------

/// The layer: "given a service, wrap it". Nothing else.
#[derive(Clone)]
pub struct TimingLayer;

impl<S> Layer<S> for TimingLayer {
    type Service = Timing<S>;

    fn layer(&self, inner: S) -> Timing<S> {
        Timing { inner }
    }
}

/// The service: "a request in, a future of a response out".
#[derive(Clone)]
pub struct Timing<S> {
    inner: S,
}

impl<S> Service<Request> for Timing<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), S::Error>> {
        self.inner.poll_ready(cx) // "are you ready?" — ask the one inside
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let started = Instant::now();
        let fut = self.inner.call(req); // the request goes further in
        Box::pin(async move {
            let mut res = fut.await?; // ...and the response comes back here
            let ms = started.elapsed().as_millis().to_string();
            let value = HeaderValue::from_str(&ms).expect("digits are a valid header value");
            res.headers_mut().insert("x-response-time-ms", value);
            Ok(res)
        })
    }
}

pub fn timing_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                tokio::time::sleep(Duration::from_millis(50)).await;
                "ok"
            }),
        )
        .layer(TimingLayer)
}

// ---------------------------------------------------------------------------
// 3. The tower-http stack
// ---------------------------------------------------------------------------

pub const REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

async fn slow() -> &'static str {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "finally"
}

async fn echo(body: String) -> String {
    format!("got {} bytes", body.len())
}

async fn report() -> Json<Vec<String>> {
    Json((1..=50).map(|i| format!("order line number {i}")).collect())
}

async fn upload_json(Json(v): Json<serde_json::Value>) -> String {
    format!("json with {} bytes", v.to_string().len())
}

/// Production-like stack. In `ServiceBuilder` the first line is the outermost.
pub fn stack_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://app.example.com"))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .max_age(Duration::from_secs(3600));

    Router::new()
        .route("/slow", get(slow))
        .route("/echo", post(echo))
        .route("/report", get(report))
        .route("/json", post(upload_json))
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(REQUEST_ID, MakeRequestUuid))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|req: &Request| {
                            let id = req.headers().get(&REQUEST_ID);
                            let id = id.and_then(|v| v.to_str().ok()).unwrap_or("-");
                            tracing::info_span!("http", method = %req.method(), uri = %req.uri(), id)
                        })
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(PropagateRequestIdLayer::new(REQUEST_ID))
                .layer(cors)
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(2),
                ))
                .layer(RequestBodyLimitLayer::new(1024)),
        )
}

/// The trap: tower-http's limit is 10 MB, but `Json` still stops at axum's 2 MB.
pub fn body_limit_trap() -> Router {
    Router::new()
        .route("/json", post(upload_json))
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
}

/// The fix: switch off axum's own default, so only one limit is left.
pub fn body_limit_fixed() -> Router {
    Router::new()
        .route("/json", post(upload_json))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
}

// ---------------------------------------------------------------------------
// 4. route_layer vs layer
// ---------------------------------------------------------------------------

/// A token check as a middleware: no token -> 401 before the handler.
pub async fn require_token(req: Request, next: Next) -> Response {
    let ok = req.headers().get(header::AUTHORIZATION)
        == Some(&HeaderValue::from_static("Bearer s3cret"));
    if !ok {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(req).await
}

/// `.layer()`: the token check also runs for unknown paths -> 401 instead of 404.
pub fn with_layer() -> Router {
    Router::new()
        .route("/admin", get(|| async { "secret" }))
        .layer(middleware::from_fn(require_token))
}

/// `.route_layer()`: only for matched routes; unknown paths stay 404.
pub fn with_route_layer() -> Router {
    Router::new()
        .route("/admin", get(|| async { "secret" }))
        .route_layer(middleware::from_fn(require_token))
}
