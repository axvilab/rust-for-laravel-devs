//! Exercise 3: post a form, flash "order created", redirect (303), show the message exactly once
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
// `cargo test -p ch16g-axum-deeper --example exercise-3` runs the test below.

use axum::{Form, Router, response::Redirect, routing::get};
use ch16g_axum_deeper::sessions::{flash, take_flash};
use serde::Deserialize;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

#[derive(Deserialize)]
struct NewOrder {
    sku: String,
}

async fn create(session: Session, Form(order): Form<NewOrder>) -> Redirect {
    // ... save the order ...
    flash(
        &session,
        "success",
        &format!("order for {} created", order.sku),
    )
    .await;
    Redirect::to("/orders") // 303 See Other: the browser follows it with GET
}

async fn index(session: Session) -> String {
    match take_flash(&session).await {
        Some(f) => format!("[{}] {}\norders: ...", f.level, f.text),
        None => "orders: ...".into(),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/orders", get(index).post(create))
        .layer(SessionManagerLayer::new(MemoryStore::default()).with_secure(false))
}

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(app()).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, header},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn body(app: &Router, req: Request<Body>) -> (String, Option<String>) {
        let res = app.clone().oneshot(req).await.unwrap();
        let cookie = res
            .headers()
            .get(header::SET_COOKIE)
            .map(|v| v.to_str().unwrap().split(';').next().unwrap().to_owned());
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        (String::from_utf8(bytes.to_vec()).unwrap(), cookie)
    }

    #[tokio::test]
    async fn flash_is_shown_once() {
        let app = app();
        let post = Request::post("/orders")
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from("sku=BOOK-1"))
            .unwrap();
        let (_, cookie) = body(&app, post).await;
        let cookie = cookie.expect("the session cookie");
        let get = || {
            Request::get("/orders")
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap()
        };
        assert_eq!(
            body(&app, get()).await.0,
            "[success] order for BOOK-1 created\norders: ..."
        );
        assert_eq!(body(&app, get()).await.0, "orders: ...");
    }
}
