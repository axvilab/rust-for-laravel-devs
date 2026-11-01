//! `.layer()` vs `.route_layer()`: an auth middleware turns unknown paths into 401 or leaves them 404
// Runs both routers through `oneshot`. No server needed.

use axum::{Router, body::Body, http::Request};
use ch16g_axum_deeper::layers::{with_layer, with_route_layer};
use tower::ServiceExt;

async fn status(app: Router, uri: &str) -> u16 {
    let req = Request::get(uri).body(Body::empty()).unwrap();
    app.oneshot(req).await.unwrap().status().as_u16()
}

#[tokio::main]
async fn main() {
    for uri in ["/admin", "/nope"] {
        println!(
            "GET {uri:7} layer: {}   route_layer: {}",
            status(with_layer(), uri).await,
            status(with_route_layer(), uri).await
        );
    }
}
