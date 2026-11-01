//! Cookies with axum-extra: a plain `CookieJar`, a `SignedCookieJar` (readable, tamper-proof)
//! and a `PrivateCookieJar` (encrypted).

use axum::{
    Router,
    extract::{FromRef, Path},
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use axum_extra::extract::cookie::{
    Cookie, CookieJar, Key, PrivateCookieJar, SameSite, SignedCookieJar,
};

/// The key signs and encrypts cookies, like `APP_KEY` in Laravel.
#[derive(Clone)]
pub struct AppState {
    pub key: Key,
}

/// Signed and private jars find the key in the state through this impl.
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Key {
        state.key.clone()
    }
}

impl AppState {
    /// `COOKIE_KEY` must be at least 64 bytes; generate one with `openssl rand -base64 64`.
    /// Without it we make a random key: fine for a demo, but every restart logs everyone out.
    pub fn from_env() -> Self {
        let key = match std::env::var("COOKIE_KEY") {
            Ok(s) => Key::try_from(s.as_bytes()).expect("COOKIE_KEY must be at least 64 bytes"),
            Err(_) => Key::generate(),
        };
        Self { key }
    }
}

// --- plain: the user can read AND change it -------------------------------------

async fn set_theme(jar: CookieJar, Path(theme): Path<String>) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("theme", theme))
        .path("/") // without it the browser keeps the cookie for /theme/... only
        .same_site(SameSite::Lax)
        .permanent(); // Max-Age of 20 years, like Cookie::forever() in Laravel
    (jar.add(cookie), Redirect::to("/"))
}

async fn show_theme(jar: CookieJar) -> String {
    let theme = jar.get("theme").map(|c| c.value().to_owned());
    format!("theme: {}", theme.as_deref().unwrap_or("light"))
}

// --- signed: readable in the browser, but any change breaks the signature ------

async fn remember(jar: SignedCookieJar) -> (SignedCookieJar, &'static str) {
    let visitor = uuid::Uuid::new_v4().to_string();
    (jar.add(Cookie::new("visitor", visitor)), "remembered")
}

async fn whoami(jar: SignedCookieJar) -> String {
    match jar.get("visitor") {
        Some(c) => format!("visitor {}", c.value()),
        None => "a stranger (no cookie, or it was tampered with)".into(),
    }
}

// --- private: encrypted, the browser sees only noise -----------------------------

async fn add_to_cart(jar: PrivateCookieJar, Path(sku): Path<String>) -> impl IntoResponse {
    let mut cart: Vec<String> = jar
        .get("cart")
        .and_then(|c| serde_json::from_str(c.value()).ok())
        .unwrap_or_default();
    cart.push(sku);
    let value = serde_json::to_string(&cart).expect("Vec<String> always serializes");
    let cookie = Cookie::build(("cart", value)).path("/").http_only(true);
    (jar.add(cookie), format!("cart: {}", cart.join(", ")))
}

async fn clear_cart(jar: PrivateCookieJar) -> (PrivateCookieJar, &'static str) {
    // the path must match the one the cookie was set with, or the browser keeps it
    (jar.remove(Cookie::build("cart").path("/")), "cart cleared")
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(show_theme))
        .route("/theme/{name}", post(set_theme))
        .route("/remember", post(remember))
        .route("/whoami", get(whoami))
        .route("/cart/{sku}", post(add_to_cart))
        .route("/cart", axum::routing::delete(clear_cart))
        .with_state(state)
}
