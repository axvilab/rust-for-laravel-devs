//! Extractors, deeper: AppPath/AppJson with JSON rejections, ApiVersion, Option and Result, Extension
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -si localhost:3000/orders/abc               → 400 {"error":"Invalid URL: ..."}
//   curl -s localhost:3000/orders/7 -H 'x-api-version: 2'

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::extractors::app()).await;
}
