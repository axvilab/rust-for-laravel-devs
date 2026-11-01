mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ch16g_axum_deeper::extractors;
use common::{get, send};

fn post_json(uri: &str, body: &'static str) -> Request<Body> {
    Request::post(uri)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

#[tokio::test]
async fn path_rejection_is_json() {
    let res = send(extractors::app(), get("/orders/abc")).await;
    assert_eq!(res.status, StatusCode::BAD_REQUEST);
    assert_eq!(res.headers["content-type"], "application/json");
    assert_eq!(res.body, r#"{"error":"Cannot parse `abc` to a `u64`"}"#);
}

#[tokio::test]
async fn json_rejections_keep_their_status() {
    let app = extractors::app();
    let no_type = Request::post("/orders").body(Body::from("{}")).unwrap();
    assert_eq!(
        send(app.clone(), no_type).await.status,
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    );
    assert_eq!(
        send(app.clone(), post_json("/orders", "{bad")).await.status,
        StatusCode::BAD_REQUEST
    );
    let res = send(app.clone(), post_json("/orders", r#"{"sku":"A"}"#)).await;
    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(res.body.contains("missing field `qty`"), "{}", res.body);
    let res = send(app, post_json("/orders", r#"{"sku":"A","qty":2}"#)).await;
    assert_eq!(res.status, StatusCode::CREATED);
    assert_eq!(res.body, r#"{"sku":"A","qty":2}"#);
}

#[tokio::test]
async fn api_version_extractor() {
    let app = extractors::app();
    assert_eq!(
        send(app.clone(), get("/orders/7")).await.body,
        r#"{"id":7}"#
    );
    let v2 = Request::get("/orders/7")
        .header("x-api-version", "2")
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        send(app.clone(), v2).await.body,
        r#"{"data":{"id":7},"version":2}"#
    );
    let v9 = Request::get("/orders/7")
        .header("x-api-version", "9")
        .body(Body::empty())
        .unwrap();
    assert_eq!(send(app, v9).await.status, StatusCode::NOT_ACCEPTABLE);
}

#[tokio::test]
async fn result_and_option_extractors() {
    let app = extractors::app();
    let plain = Request::post("/lenient").body(Body::from("x")).unwrap();
    assert_eq!(send(app.clone(), plain).await.body, "send JSON, please");
    assert_eq!(
        send(app.clone(), post_json("/lenient", "{}")).await.body,
        "bad body (422 Unprocessable Entity), using defaults"
    );
    let empty = Request::post("/optional").body(Body::empty()).unwrap();
    assert_eq!(send(app.clone(), empty).await.body, "no body, fine");
    // Option does not hide a broken body
    assert_eq!(
        send(app, post_json("/optional", "{bad")).await.status,
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn extension_from_middleware() {
    let req = Request::get("/tenant")
        .header("host", "acme.shop.test")
        .body(Body::empty())
        .unwrap();
    assert_eq!(send(extractors::app(), req).await.body, "tenant: acme");

    let res = send(extractors::app_without_tenant_middleware(), get("/tenant")).await;
    assert_eq!(res.status, StatusCode::INTERNAL_SERVER_ERROR);
    println!("{}", res.body);
    assert!(
        res.body.starts_with("Missing request extension"),
        "{}",
        res.body
    );
}
