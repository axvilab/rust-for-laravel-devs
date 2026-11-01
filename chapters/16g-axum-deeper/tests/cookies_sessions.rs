mod common;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
    routing,
};
use axum_extra::extract::cookie::Key;
use ch16g_axum_deeper::{cookies, sessions};
use common::{get, send};
use tower_sessions::Session;

fn cookie_app() -> Router {
    cookies::app(cookies::AppState {
        key: Key::generate(),
    })
}

/// "name=value" from the first set-cookie header with that name.
fn cookie(res: &common::Res, name: &str) -> String {
    res.headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap())
        .find(|v| v.starts_with(&format!("{name}=")))
        .map(|v| v.split(';').next().unwrap().to_owned())
        .unwrap_or_else(|| panic!("no {name} cookie"))
}

fn with_cookie(method: &str, uri: &str, cookie: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, cookie)
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn plain_cookie_has_path_and_samesite() {
    let res = send(
        cookie_app(),
        Request::post("/theme/dark").body(Body::empty()).unwrap(),
    )
    .await;
    assert_eq!(res.status, StatusCode::SEE_OTHER);
    let raw = res.headers[header::SET_COOKIE].to_str().unwrap();
    assert!(
        raw.starts_with("theme=dark; SameSite=Lax; Path=/; Max-Age="),
        "{raw}"
    );
    let res = send(cookie_app(), with_cookie("GET", "/", "theme=dark")).await;
    assert_eq!(res.body, "theme: dark");
}

#[tokio::test]
async fn signed_cookie_rejects_tampering() {
    let app = cookie_app();
    let res = send(
        app.clone(),
        Request::post("/remember").body(Body::empty()).unwrap(),
    )
    .await;
    let good = cookie(&res, "visitor");
    let res = send(app.clone(), with_cookie("GET", "/whoami", &good)).await;
    assert!(res.body.starts_with("visitor "), "{}", res.body);

    // change the last character of the value (the visitor id, readable in plain text)
    let mut bad = good.clone();
    let last = bad.pop().unwrap();
    bad.push(if last == '0' { '1' } else { '0' });
    let res = send(app, with_cookie("GET", "/whoami", &bad)).await;
    assert!(res.body.starts_with("a stranger"), "{}", res.body);
}

#[tokio::test]
async fn private_cookie_round_trip_and_other_key_fails() {
    let app = cookie_app();
    let res = send(
        app.clone(),
        Request::post("/cart/BOOK-1").body(Body::empty()).unwrap(),
    )
    .await;
    let cart = cookie(&res, "cart");
    assert!(!cart.contains("BOOK-1"), "encrypted: {cart}");
    let res = send(app, with_cookie("POST", "/cart/MUG-2", &cart)).await;
    assert_eq!(res.body, "cart: BOOK-1, MUG-2");

    // a server with another key (e.g. a restart with Key::generate()) can't read it
    let res = send(cookie_app(), with_cookie("POST", "/cart/MUG-2", &cart)).await;
    assert_eq!(res.body, "cart: MUG-2");
}

#[tokio::test]
async fn short_key_is_an_error_not_a_panic() {
    assert!(Key::try_from(&[0u8; 32][..]).is_err());
    assert!(Key::try_from(&[0u8; 64][..]).is_ok());
}

#[tokio::test]
async fn session_counter_and_flash_once() {
    let app = sessions::app();
    let res = send(app.clone(), get("/visits")).await;
    assert_eq!(res.body, "visit number 1");
    let sid = cookie(&res, "app_session");
    let res = send(app.clone(), with_cookie("GET", "/visits", &sid)).await;
    assert_eq!(res.body, "visit number 2");

    // a wrong password leaves a flash message for the next request only
    let req = Request::post("/login")
        .header(header::COOKIE, &sid)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from("email=v%40example.com&password=nope"))
        .unwrap();
    assert_eq!(send(app.clone(), req).await.status, StatusCode::SEE_OTHER);
    let res = send(app.clone(), with_cookie("GET", "/login", &sid)).await;
    assert_eq!(res.body, "login form [error: wrong email or password]");
    let res = send(app, with_cookie("GET", "/login", &sid)).await;
    assert_eq!(res.body, "login form");
}

#[tokio::test]
async fn login_cycles_the_session_id() {
    let app = sessions::app();
    let sid = cookie(&send(app.clone(), get("/visits")).await, "app_session");
    let req = Request::post("/login")
        .header(header::COOKIE, &sid)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from("email=v%40example.com&password=secret"))
        .unwrap();
    let res = send(app.clone(), req).await;
    let new_sid = cookie(&res, "app_session");
    assert_ne!(sid, new_sid);

    let res = send(app.clone(), with_cookie("GET", "/dashboard", &new_sid)).await;
    assert_eq!(res.body, "dashboard of v@example.com welcome back");
    // the old id is dead: back to the login page
    let res = send(app, with_cookie("GET", "/dashboard", &sid)).await;
    assert_eq!(res.headers[header::LOCATION], "/login");
}

#[tokio::test]
async fn csrf_layer_blocks_cross_site_posts() {
    let app = sessions::app();
    let cross = Request::post("/logout")
        .header("sec-fetch-site", "cross-site")
        .body(Body::empty())
        .unwrap();
    assert_eq!(send(app.clone(), cross).await.status, StatusCode::FORBIDDEN);

    let foreign_origin = Request::post("/logout")
        .header(header::HOST, "localhost:3000")
        .header(header::ORIGIN, "https://evil.example")
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        send(app.clone(), foreign_origin).await.status,
        StatusCode::FORBIDDEN
    );

    // curl and server-to-server calls send neither header: allowed
    let plain = Request::post("/logout").body(Body::empty()).unwrap();
    assert_eq!(send(app, plain).await.status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn session_without_layer_is_500() {
    async fn h(_s: Session) -> &'static str {
        "never"
    }
    let app = Router::new().route("/", routing::get(h));
    let res = send(app, get("/")).await;
    assert_eq!(res.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        res.body,
        "Can't extract session. Is `SessionManagerLayer` enabled?"
    );
}
