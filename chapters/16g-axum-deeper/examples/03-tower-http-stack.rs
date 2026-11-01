//! request id, TraceLayer, CORS, compression, timeout, body limit — one ServiceBuilder
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::layers::stack_router()).await;
}
