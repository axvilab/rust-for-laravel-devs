//! tower_governor with a custom JSON 429 response
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::limits::rate_limited_json()).await;
}
