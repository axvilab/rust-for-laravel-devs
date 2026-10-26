//! Integration tests against a real Postgres.
//!
//! `#[sqlx::test]` creates a fresh database per test, runs the migrations and
//! drops it afterwards. It needs `DATABASE_URL` pointing at a server where the
//! user may `CREATE DATABASE`, e.g.:
//!
//!   docker compose up -d db
//!   DATABASE_URL=postgres://postgres:pg@localhost:5432/postgres cargo test
//!
//! Requests go through the real `Router` (all middleware included) via
//! `tower::ServiceExt::oneshot` — no TCP port, no running server.

use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Method, Request, StatusCode, header},
};
use http_body_util::BodyExt;
use orders_api::{
    app,
    config::Config,
    domain::{Email, Order, OrderItem},
    error::AppError,
    repo::OrderRepo,
    state::AppState,
};
use rust_decimal_macros::dec;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

fn test_config() -> Config {
    Config {
        database_url: "unused: the pool comes from #[sqlx::test]".into(),
        jwt_secret: "test-secret".into(),
        host: "127.0.0.1".into(),
        port: 0,
        log_json: false,
        db_max_connections: 5,
        request_timeout_secs: 10,
        cors_origins: String::new(),
        jwt_ttl_hours: 1,
    }
}

fn test_app(pool: PgPool) -> Router {
    app(AppState::new(pool, test_config()))
}

struct Res {
    status: StatusCode,
    headers: HeaderMap,
    body: Value,
}

async fn call(
    app: &Router,
    method: Method,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> Res {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(t) = token {
        req = req.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    let req = match body {
        Some(b) => req
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(b.to_string())),
        None => req.body(Body::empty()),
    }
    .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
    Res {
        status,
        headers,
        body,
    }
}

async fn register_and_login(app: &Router, email: &str) -> String {
    let r = call(
        app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(json!({ "email": email, "password": "password123" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::CREATED, "{}", r.body);

    let r = call(
        app,
        Method::POST,
        "/api/v1/auth/login",
        None,
        Some(json!({ "email": email, "password": "password123" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "{}", r.body);
    r.body["token"].as_str().unwrap().to_owned()
}

async fn create_order(app: &Router, token: &str, items: Value) -> Value {
    let r = call(
        app,
        Method::POST,
        "/api/v1/orders",
        Some(token),
        Some(json!({ "customer_email": "buyer@example.com", "items": items })),
    )
    .await;
    assert_eq!(r.status, StatusCode::CREATED, "{}", r.body);
    r.body
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn health_ready_and_fallback(pool: PgPool) {
    let app = test_app(pool);
    let r = call(&app, Method::GET, "/health", None, None).await;
    assert_eq!((r.status, r.body), (StatusCode::OK, json!("ok")));

    let r = call(&app, Method::GET, "/ready", None, None).await;
    assert_eq!((r.status, r.body), (StatusCode::OK, json!("ready")));

    let r = call(&app, Method::GET, "/nope", None, None).await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    assert_eq!(r.body["error"], "not found");
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn request_id_is_generated_or_propagated(pool: PgPool) {
    let app = test_app(pool);
    let r = call(&app, Method::GET, "/health", None, None).await;
    let id = r.headers.get("x-request-id").expect("x-request-id header");
    assert!(Uuid::parse_str(id.to_str().unwrap()).is_ok());

    let req = Request::get("/health")
        .header("x-request-id", "from-proxy-42")
        .body(Body::empty())
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.headers()["x-request-id"], "from-proxy-42");
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn auth_flow(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "Vitaly@Example.com").await;

    // duplicate email (case-insensitive thanks to Email normalization) -> 409
    let r = call(
        &app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(json!({ "email": "vitaly@example.com", "password": "password123" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::CONFLICT);

    // wrong password / unknown user -> 401
    for (email, pw) in [
        ("vitaly@example.com", "wrong-password"),
        ("ghost@example.com", "password123"),
    ] {
        let r = call(
            &app,
            Method::POST,
            "/api/v1/auth/login",
            None,
            Some(json!({ "email": email, "password": pw })),
        )
        .await;
        assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    }

    // short password -> 422 with field errors
    let r = call(
        &app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(json!({ "email": "x@example.com", "password": "short" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(r.body["fields"]["password"].is_array(), "{}", r.body);

    // Option<AuthUser>: anonymous ok, valid token ok, garbage token 401
    let r = call(&app, Method::GET, "/api/v1/me", None, None).await;
    assert_eq!(r.body["role"], "guest");
    let r = call(&app, Method::GET, "/api/v1/me", Some(&token), None).await;
    assert_eq!(r.body["role"], "user");
    let r = call(&app, Method::GET, "/api/v1/me", Some("garbage"), None).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn orders_require_auth(pool: PgPool) {
    let app = test_app(pool);
    let r = call(
        &app,
        Method::POST,
        "/api/v1/orders",
        None,
        Some(json!({ "customer_email": "a@b.co" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);

    let r = call(&app, Method::GET, "/api/v1/orders", Some("not.a.jwt"), None).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn order_lifecycle(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "v@example.com").await;

    // empty order cannot be paid -> 422 with the domain message
    let empty = create_order(&app, &token, json!([])).await;
    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{}/pay", empty["id"].as_str().unwrap()),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(r.body["error"], "cannot pay an empty order");

    // create with one item, then add another one
    let o = create_order(
        &app,
        &token,
        json!([{ "sku": "SKU-1", "qty": 2, "unit_price": "100.00" }]),
    )
    .await;
    assert_eq!(o["status"], "draft");
    assert_eq!(o["total"], "200.00");
    let id = o["id"].as_str().unwrap();

    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/items"),
        Some(&token),
        Some(json!({ "sku": "SKU-2", "qty": 3, "unit_price": 0.5 })),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "{}", r.body);
    assert_eq!(r.body["total"], "201.50");
    // money always has two decimals, even if the client sent `0.5`
    assert_eq!(r.body["items"][1]["unit_price"], "0.50");

    // pay -> paid; second pay -> 422; adding to a paid order -> 422
    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/pay"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(
        (r.status, r.body["status"].clone()),
        (StatusCode::OK, json!("paid"))
    );

    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/pay"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        r.body["error"],
        "order is already paid and cannot be changed"
    );

    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/items"),
        Some(&token),
        Some(json!({ "sku": "SKU-3", "qty": 1, "unit_price": "1" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);

    // state survives a round trip through the database
    let r = call(
        &app,
        Method::GET,
        &format!("/api/v1/orders/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.body["status"], "paid");
    assert_eq!(r.body["items"].as_array().unwrap().len(), 2);
    assert_eq!(r.body["items"][0]["sku"], "SKU-1");

    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/cancel"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.body["status"], "cancelled");
    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/cancel"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn validation_and_rejections(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "v@example.com").await;

    // qty 0, bad sku, 3 decimals -> 422 with a structured field list
    let r = call(
        &app,
        Method::POST,
        "/api/v1/orders",
        Some(&token),
        Some(json!({
            "customer_email": "a@b.co",
            "items": [{ "sku": "bad sku", "qty": 0, "unit_price": "1.001" }]
        })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(r.body["error"], "validation");
    assert!(r.body["fields"]["items"].is_object(), "{}", r.body);

    // invalid email is rejected by the Email newtype during deserialization
    let r = call(
        &app,
        Method::POST,
        "/api/v1/orders",
        Some(&token),
        Some(json!({ "customer_email": "nope" })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(r.body["error"].as_str().unwrap().contains("invalid email"));

    // negative qty: u32 cannot be negative -> 422 from deserialization
    let r = call(
        &app,
        Method::POST,
        "/api/v1/orders",
        Some(&token),
        Some(json!({ "customer_email": "a@b.co", "items": [{ "sku": "A", "qty": -5, "unit_price": 1 }] })),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);

    // broken JSON -> 400
    let req = Request::post("/api/v1/orders")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{not json"))
        .unwrap();
    assert_eq!(
        app.clone().oneshot(req).await.unwrap().status(),
        StatusCode::BAD_REQUEST
    );

    // pagination bounds -> 422, unknown sort -> 400, bad uuid -> 400, unknown id -> 404
    for (uri, expected) in [
        (
            "/api/v1/orders?per_page=1000",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        ("/api/v1/orders?page=0", StatusCode::UNPROCESSABLE_ENTITY),
        ("/api/v1/orders?per_page=-5", StatusCode::BAD_REQUEST),
        (
            "/api/v1/orders?sort=id;drop%20table%20orders",
            StatusCode::BAD_REQUEST,
        ),
        ("/api/v1/orders/not-a-uuid", StatusCode::BAD_REQUEST),
    ] {
        let r = call(&app, Method::GET, uri, Some(&token), None).await;
        assert_eq!(r.status, expected, "{uri}: {}", r.body);
    }
    let r = call(
        &app,
        Method::GET,
        &format!("/api/v1/orders/{}", Uuid::new_v4()),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn pagination_filters_and_sorting(pool: PgPool) {
    let app = test_app(pool);
    let token = register_and_login(&app, "v@example.com").await;

    let cheap = create_order(
        &app,
        &token,
        json!([{ "sku": "A", "qty": 1, "unit_price": "1.00" }]),
    )
    .await;
    let pricey = create_order(
        &app,
        &token,
        json!([{ "sku": "B", "qty": 1, "unit_price": "999.00" }]),
    )
    .await;
    let mid = create_order(
        &app,
        &token,
        json!([{ "sku": "C", "qty": 5, "unit_price": "10.00" }]),
    )
    .await;
    let paid_id = mid["id"].as_str().unwrap();
    call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{paid_id}/pay"),
        Some(&token),
        None,
    )
    .await;

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?per_page=2",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(
        r.body["meta"],
        json!({ "page": 1, "per_page": 2, "total": 3, "last_page": 2 })
    );
    assert_eq!(r.body["data"].as_array().unwrap().len(), 2);
    // items are loaded for every order in the page (ANY($1), not N+1)
    assert_eq!(r.body["data"][0]["items"].as_array().unwrap().len(), 1);

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?per_page=2&page=2",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.body["data"].as_array().unwrap().len(), 1);

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?status=paid",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.body["meta"]["total"], 1);
    assert_eq!(r.body["data"][0]["id"], mid["id"]);

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?sort=total_desc",
        Some(&token),
        None,
    )
    .await;
    let ids: Vec<&Value> = r.body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| &o["id"])
        .collect();
    assert_eq!(ids, vec![&pricey["id"], &mid["id"], &cheap["id"]]);

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?sort=created_asc",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.body["data"][0]["id"], cheap["id"]);

    let r = call(
        &app,
        Method::GET,
        "/api/v1/orders?customer_email=BUYER@example.com&created_after=2000-01-01T00:00:00Z",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(r.body["meta"]["total"], 3);
}

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn users_are_isolated_and_admin_sees_all(pool: PgPool) {
    let app = test_app(pool.clone());
    let alice = register_and_login(&app, "alice@example.com").await;
    let bob = register_and_login(&app, "bob@example.com").await;

    let o = create_order(&app, &alice, json!([])).await;
    let id = o["id"].as_str().unwrap();

    // Bob cannot see or touch Alice's order: 404, not 403 (no id leaking)
    let r = call(
        &app,
        Method::GET,
        &format!("/api/v1/orders/{id}"),
        Some(&bob),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    let r = call(
        &app,
        Method::POST,
        &format!("/api/v1/orders/{id}/cancel"),
        Some(&bob),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    let r = call(&app, Method::GET, "/api/v1/orders", Some(&bob), None).await;
    assert_eq!(r.body["meta"]["total"], 0);

    // regular user -> 403 on the admin route
    let r = call(&app, Method::GET, "/api/v1/admin/orders", Some(&bob), None).await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);

    // promote Bob; the role is in the token, so he logs in again
    sqlx::query("UPDATE users SET role = 'admin' WHERE email = 'bob@example.com'")
        .execute(&pool)
        .await
        .unwrap();
    let r = call(
        &app,
        Method::POST,
        "/api/v1/auth/login",
        None,
        Some(json!({ "email": "bob@example.com", "password": "password123" })),
    )
    .await;
    let admin = r.body["token"].as_str().unwrap().to_owned();

    let r = call(
        &app,
        Method::GET,
        "/api/v1/admin/orders",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.body["meta"]["total"], 1);
    let r = call(
        &app,
        Method::GET,
        &format!("/api/v1/orders/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
}

/// RAII rollback: the order row is inserted, then the items INSERT fails
/// (`numeric(12, 2)` overflow). `?` returns early, the transaction is dropped
/// without `commit()`, and the order row disappears with it.
#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn failed_save_rolls_back(pool: PgPool) {
    let user_id = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, password_hash) VALUES ($1, 'u@example.com', 'x')")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    let repo = OrderRepo::new(pool.clone());
    let mut order = Order::new(user_id, Email::try_from("a@b.co").unwrap(), None);
    order
        .add_item(OrderItem {
            sku: "A".into(),
            qty: 1,
            unit_price: dec!(10_000_000_000_000),
        })
        .unwrap();

    let err = repo.save(&order).await.unwrap_err();
    assert!(matches!(err, AppError::Db(_)), "{err:?}");
    assert!(matches!(repo.find(order.id).await, Err(AppError::NotFound)));

    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM orders")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, 0);
}
