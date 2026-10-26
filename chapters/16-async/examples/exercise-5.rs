//! Вправа 5: перший HTTP-сервер — `/hello/{name}`
// long-running: http://localhost:3000/hello/Віталій → «Привіт, Віталій!», Ctrl+C — зупинити

use axum::{Router, extract::Path, routing::get};

async fn hello(Path(name): Path<String>) -> String {
    format!("Привіт, {name}!")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/{name}", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("слухаю :3000");
    axum::serve(listener, app).await.unwrap();
}
