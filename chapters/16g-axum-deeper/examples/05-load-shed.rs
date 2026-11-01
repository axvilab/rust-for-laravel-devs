//! tower middleware behind HandleErrorLayer: load_shed + concurrency_limit + timeout
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::limits::overload_router()).await;
}
