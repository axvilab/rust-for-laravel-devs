mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ch16g_axum_deeper::{layers, limits};
use common::{get, send};
use tower::ServiceExt;

fn layer_names(res: &common::Res) -> Vec<String> {
    res.headers
        .get_all("x-layers")
        .iter()
        .map(|v| v.to_str().unwrap().to_owned())
        .collect()
}

#[tokio::test]
async fn router_layer_last_added_is_outermost() {
    let res = send(layers::order_router(), get("/")).await;
    // headers are appended on the way OUT: innermost first
    assert_eq!(layer_names(&res), ["A", "B", "C"]);
}

#[tokio::test]
async fn service_builder_reads_top_to_bottom() {
    let res = send(layers::order_builder(), get("/")).await;
    assert_eq!(layer_names(&res), ["C", "B", "A"]);
}

#[tokio::test]
async fn hand_written_layer_adds_timing_header() {
    let res = send(layers::timing_router(), get("/")).await;
    let ms: u64 = res.headers["x-response-time-ms"]
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(ms >= 50, "{ms}");
}

#[tokio::test]
async fn stack_sets_or_keeps_request_id_and_compresses() {
    let res = send(layers::stack_router(), get("/report")).await;
    assert_eq!(res.headers["x-request-id"].len(), 36); // a fresh UUID

    let req = Request::get("/report")
        .header("x-request-id", "abc-123")
        .header("accept-encoding", "gzip")
        .body(Body::empty())
        .unwrap();
    let res = send(layers::stack_router(), req).await;
    assert_eq!(res.headers["x-request-id"], "abc-123");
    assert_eq!(res.headers["content-encoding"], "gzip");
}

#[tokio::test]
async fn stack_times_out_with_408() {
    let res = send(layers::stack_router(), get("/slow")).await;
    assert_eq!(res.status, StatusCode::REQUEST_TIMEOUT);
}

#[tokio::test]
async fn stack_limits_body_to_1kb() {
    let req = Request::post("/echo")
        .body(Body::from("a".repeat(2000)))
        .unwrap();
    let res = send(layers::stack_router(), req).await;
    assert_eq!(res.status, StatusCode::PAYLOAD_TOO_LARGE);

    let req = Request::post("/echo")
        .body(Body::from("a".repeat(500)))
        .unwrap();
    let res = send(layers::stack_router(), req).await;
    assert_eq!(res.body, "got 500 bytes");
}

fn three_mb_json() -> Request<Body> {
    let body = format!("{{\"data\":\"{}\"}}", "x".repeat(3 * 1024 * 1024));
    Request::post("/json")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

#[tokio::test]
async fn axum_default_limit_still_applies_under_tower_http_limit() {
    let res = send(layers::body_limit_trap(), three_mb_json()).await;
    assert_eq!(res.status, StatusCode::PAYLOAD_TOO_LARGE);
    println!("{}", res.body);
    assert!(res.body.contains("length limit exceeded"), "{}", res.body);

    let res = send(layers::body_limit_fixed(), three_mb_json()).await;
    assert_eq!(res.status, StatusCode::OK, "{}", res.body);
}

#[tokio::test]
async fn layer_vs_route_layer_on_unknown_path() {
    assert_eq!(
        send(layers::with_layer(), get("/nope")).await.status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        send(layers::with_route_layer(), get("/nope")).await.status,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        send(layers::with_route_layer(), get("/admin")).await.status,
        StatusCode::UNAUTHORIZED
    );
}

async fn three_at_once(app: axum::Router) -> Vec<u16> {
    let calls = (0..3).map(|_| app.clone().oneshot(get("/report")));
    let mut statuses: Vec<u16> = futures_util::future::join_all(calls)
        .await
        .into_iter()
        .map(|r| r.unwrap().status().as_u16())
        .collect();
    statuses.sort();
    statuses
}

#[tokio::test]
async fn load_shed_answers_503_instead_of_queueing() {
    // `.with_state(())` turns handlers into routes ONCE, as `axum::serve` does.
    let app = limits::overload_router().with_state(());
    assert_eq!(three_at_once(app).await, [200, 200, 503]);
}

#[tokio::test]
async fn without_with_state_oneshot_rebuilds_layers_per_request() {
    // The trap: a bare `Router<()>` in `oneshot` builds the handler's layers anew for
    // every request, so each request gets its own semaphore and nobody is limited.
    assert_eq!(
        three_at_once(limits::overload_router()).await,
        [200, 200, 200]
    );
}

#[tokio::test]
async fn governor_without_connect_info_is_500() {
    let res = send(limits::rate_limited_router(), get("/login")).await;
    assert_eq!(res.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(res.body, "Unable To Extract Key!");
}

fn from_ip(uri: &str, ip: [u8; 4]) -> Request<Body> {
    use axum::extract::ConnectInfo;
    use std::net::SocketAddr;
    let mut req = get(uri);
    // what `into_make_service_with_connect_info` puts into every real request
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from((ip, 5555))));
    req
}

#[tokio::test]
async fn governor_allows_burst_then_429() {
    let app = limits::rate_limited_json();
    let mut statuses = Vec::new();
    for _ in 0..4 {
        statuses.push(
            send(app.clone(), from_ip("/login", [10, 0, 0, 1]))
                .await
                .status
                .as_u16(),
        );
    }
    assert_eq!(statuses, [200, 200, 200, 429]);
    let res = send(app.clone(), from_ip("/login", [10, 0, 0, 1])).await;
    assert!(
        res.body
            .starts_with(r#"{"error":"too many requests, retry in "#),
        "{}",
        res.body
    );
    // another IP has its own bucket
    let res = send(app, from_ip("/login", [10, 0, 0, 2])).await;
    assert_eq!(res.status, StatusCode::OK);
}

#[tokio::test]
async fn mock_connect_info_is_not_enough_for_governor() {
    use axum::extract::connect_info::MockConnectInfo;
    use std::net::SocketAddr;
    // MockConnectInfo feeds axum's `ConnectInfo` EXTRACTOR, but tower_governor reads
    // the request extensions directly and does not see it.
    let app = limits::rate_limited_router()
        .layer(MockConnectInfo(SocketAddr::from(([10, 0, 0, 1], 5555))));
    assert_eq!(
        send(app, get("/login")).await.status,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn governor_headers() {
    let res = send(
        limits::rate_limited_router(),
        from_ip("/login", [10, 0, 0, 3]),
    )
    .await;
    assert_eq!(res.headers["x-ratelimit-limit"], "3");
    assert_eq!(res.headers["x-ratelimit-remaining"], "2");
}
