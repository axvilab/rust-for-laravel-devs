//! Static files and an SPA: ServeDir, ServeFile fallback, an API with its own 404, cache headers
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -si localhost:3000/orders/42             → 200 index.html (the SPA route)
//   curl -si localhost:3000/assets/app-3f9a1c.js  → cache-control: ... immutable

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::files::app()).await;
}
