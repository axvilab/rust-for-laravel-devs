//! axum 0.8 WebSocket: `Message::Text` holds `Utf8Bytes`, not `String`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0308]: mismatched types ... expected `Utf8Bytes`, found `String`
//! Fix: `Message::Text(text.into())` — see src/realtime.rs.
// expected-error: expected `Utf8Bytes`, found `String`
// check-with: cargo

use axum::extract::ws::{Message, WebSocket};

async fn hello(mut socket: WebSocket) {
    let text = format!("hello, {}", "Vitalii");
    let _ = socket.send(Message::Text(text)).await;
}

fn main() {
    let _ = hello;
}
