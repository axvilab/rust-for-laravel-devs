//! Exercise 5: an SSE clock — one event a second — that TimeoutLayer does not cut off and Ctrl+C ends
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C;  curl -N localhost:3000/clock
// `cargo test -p ch16g-axum-deeper --example exercise-5` runs the test below.

use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::get,
};
use ch16g_axum_deeper::realtime::wait_for_shutdown;
use futures_util::{Stream, StreamExt};
use tokio::sync::watch;
use tokio_stream::wrappers::IntervalStream;
use tower_http::timeout::TimeoutLayer;

async fn clock(
    State(stop): State<watch::Receiver<bool>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let ticks = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)));
    let stream = ticks
        .enumerate()
        .map(|(n, _)| Ok(Event::default().data(format!("tick {n}"))))
        .take_until(wait_for_shutdown(stop)); // ends when the flag becomes `true`
    Sse::new(stream)
}

pub fn app(stop: watch::Receiver<bool>) -> Router {
    Router::new()
        .route("/clock", get(clock))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(2), // counts until the response HEADERS, not the whole stream
        ))
        .with_state(stop)
}

#[tokio::main]
async fn main() {
    let (stop_tx, stop_rx) = watch::channel(false);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000 — curl -N localhost:3000/clock");
    axum::serve(listener, app(stop_rx))
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            let _ = stop_tx.send(true); // open clocks finish, then the server exits
        })
        .await
        .unwrap();
    println!("bye");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn stream_outlives_the_timeout() {
        let req = axum::http::Request::get("/clock")
            .body(Body::empty())
            .unwrap();
        let (_stop_tx, stop_rx) = watch::channel(false);
        let res = app(stop_rx).oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let mut body = res.into_body();
        let mut text = String::new();
        // 4 ticks take ~3 s — longer than the 2 s timeout
        while !text.contains("tick 3") {
            let frame = body.frame().await.unwrap().unwrap();
            if let Ok(data) = frame.into_data() {
                text.push_str(&String::from_utf8_lossy(&data));
            }
        }
        assert!(text.contains("data: tick 0"), "{text}");
    }
}

#[cfg(test)]
mod shutdown_tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn stream_ends_when_the_flag_flips() {
        let (stop_tx, stop_rx) = watch::channel(false);
        let req = axum::http::Request::get("/clock")
            .body(Body::empty())
            .unwrap();
        let res = app(stop_rx).oneshot(req).await.unwrap();
        stop_tx.send(true).unwrap();
        // the whole body completes instead of ticking forever
        let all = tokio::time::timeout(Duration::from_secs(3), res.into_body().collect()).await;
        assert!(all.is_ok());
    }
}
