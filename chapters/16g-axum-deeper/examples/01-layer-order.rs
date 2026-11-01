//! The order of `.layer()`: the last one added is the outermost; `ServiceBuilder` reads top to bottom
// Runs two routers through `oneshot` and prints what each layer saw. No server needed.

use axum::{body::Body, http::Request};
use ch16g_axum_deeper::layers::{order_builder, order_router};
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    println!("Router::layer(A).layer(B).layer(C):");
    let res = order_router()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    print_layers(&res);
    println!();

    println!("ServiceBuilder::new().layer(A).layer(B).layer(C):");
    let res = order_builder()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    print_layers(&res);
}

fn print_layers(res: &axum::response::Response) {
    let names: Vec<_> = res
        .headers()
        .get_all("x-layers")
        .iter()
        .map(|v| v.to_str().unwrap())
        .collect();
    println!("x-layers in the response: {}", names.join(", "));
}
