//! Per-IP rate limiting with tower_governor (needs ConnectInfo)
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::limits::rate_limited_router()).await;
}
