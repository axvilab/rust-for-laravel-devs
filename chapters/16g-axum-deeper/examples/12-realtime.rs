//! Real time: broadcast channel + SSE (/events) + WebSocket (/ws), with a graceful shutdown that ends the streams
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -N localhost:3000/events                                   (terminal 1)
//   curl -XPOST localhost:3000/messages -H 'content-type: application/json' \
//        -d '{"user":"v","text":"hi"}'                              (terminal 2)

use ch16g_axum_deeper::realtime::{Hub, app};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let (stop_tx, stop_rx) = watch::channel(false);
    let hub = Hub::new(stop_rx);
    let sockets = hub.sockets.clone();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app(hub))
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("shutting down: closing the streams");
            let _ = stop_tx.send(true); // SSE streams see it and finish
        })
        .await
        .unwrap();
    // HTTP is done; now wait for the WebSocket tasks to say goodbye
    sockets.close();
    sockets.wait().await;
    println!("bye");
}
