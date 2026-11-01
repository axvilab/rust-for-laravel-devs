//! Exercise 2: two rate limits — /login 5 tries a minute per IP, the rest 20 in a burst
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
// `cargo test -p ch16g-axum-deeper --example exercise-2` runs the test below.

use axum::{Router, routing::get};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

pub fn app() -> Router {
    // login: a burst of 5, then one more every 12 s = 5 a minute
    let strict = GovernorConfigBuilder::default()
        .per_second(12)
        .burst_size(5)
        .finish()
        .unwrap();
    // everything else: a burst of 20, then 2 a second
    let relaxed = GovernorConfigBuilder::default()
        .per_millisecond(500)
        .burst_size(20)
        .finish()
        .unwrap();

    let login = Router::new()
        .route("/login", get(|| async { "login" }))
        .route_layer(GovernorLayer::new(strict));
    let api = Router::new()
        .route("/orders", get(|| async { "orders" }))
        .route_layer(GovernorLayer::new(relaxed));

    login.merge(api)
}

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(app()).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, extract::ConnectInfo, http::Request};
    use std::net::SocketAddr;
    use tower::ServiceExt;

    async fn hit(app: &Router, uri: &str) -> u16 {
        let mut req = Request::get(uri).body(Body::empty()).unwrap();
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([10, 0, 0, 9], 1))));
        app.clone().oneshot(req).await.unwrap().status().as_u16()
    }

    #[tokio::test]
    async fn login_is_stricter_than_the_rest() {
        let app = app();
        let mut login = Vec::new();
        for _ in 0..6 {
            login.push(hit(&app, "/login").await);
        }
        assert_eq!(login, [200, 200, 200, 200, 200, 429]);
        // separate bucket: /orders is not affected by the login attempts
        for _ in 0..20 {
            assert_eq!(hit(&app, "/orders").await, 200);
        }
        assert_eq!(hit(&app, "/orders").await, 429);
    }
}
