//! Exercise 5: the first HTTP server — `/hello/{name}`
// long-running: http://localhost:3000/hello/Vitalii → "Hello, Vitalii!", stop with Ctrl+C

use axum::{Router, extract::Path, routing::get};

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/{name}", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
