//! Cookies: plain `CookieJar`, `SignedCookieJar`, `PrivateCookieJar` and the key in the state
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -si -XPOST localhost:3000/theme/dark       → set-cookie: theme=dark; ...
//   curl -si -XPOST localhost:3000/cart/BOOK-1      → set-cookie: cart=<encrypted>; ...

use ch16g_axum_deeper::cookies::{AppState, app};

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(app(AppState::from_env())).await;
}
