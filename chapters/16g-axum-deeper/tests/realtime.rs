//! SSE and WebSocket need a real TCP server: `oneshot` can't hold a connection open.

use std::time::Duration;

use ch16g_axum_deeper::realtime::{ChatMessage, Hub, app};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::{oneshot, watch},
};
use tokio_tungstenite::tungstenite::Message;

struct Server {
    addr: std::net::SocketAddr,
    hub: Hub,
    stop_streams: watch::Sender<bool>,
    stop_server: Option<oneshot::Sender<()>>,
    handle: tokio::task::JoinHandle<()>,
}

async fn start() -> Server {
    let (stop_streams, rx) = watch::channel(false);
    let hub = Hub::new(rx);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let router = app(hub.clone());
    let handle = tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(async {
                let _ = stop_rx.await;
            })
            .await
            .unwrap();
    });
    Server {
        addr,
        hub,
        stop_streams,
        stop_server: Some(stop_tx),
        handle,
    }
}

/// Opens `GET /events` by hand and returns a line reader positioned after the headers.
async fn open_sse(addr: std::net::SocketAddr) -> BufReader<TcpStream> {
    let mut tcp = TcpStream::connect(addr).await.unwrap();
    tcp.write_all(b"GET /events HTTP/1.1\r\nhost: x\r\n\r\n")
        .await
        .unwrap();
    let mut reader = BufReader::new(tcp);
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line).await.unwrap();
        if line == "\r\n" {
            return reader;
        }
    }
}

#[tokio::test]
async fn sse_delivers_published_messages() {
    let srv = start().await;
    let mut sse = open_sse(srv.addr).await;
    tokio::time::sleep(Duration::from_millis(50)).await; // let the subscription happen

    let msg = ChatMessage {
        user: "v".into(),
        text: "hi".into(),
    };
    assert_eq!(srv.hub.tx.send(msg).unwrap(), 1);

    let mut seen = String::new();
    while !seen.contains("data: ") {
        let mut line = String::new();
        tokio::time::timeout(Duration::from_secs(2), sse.read_line(&mut line))
            .await
            .unwrap()
            .unwrap();
        seen.push_str(&line);
    }
    assert!(seen.contains("event: message"), "{seen}");
    assert!(seen.contains(r#"data: {"user":"v","text":"hi"}"#), "{seen}");
}

#[tokio::test]
async fn graceful_shutdown_waits_for_open_sse_until_streams_end() {
    let mut srv = start().await;
    let _sse = open_sse(srv.addr).await;

    srv.stop_server.take().unwrap().send(()).unwrap();
    // the SSE connection is still open: the server keeps waiting
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(
        !srv.handle.is_finished(),
        "server must wait for the open stream"
    );

    // end the streams -> the last connection closes -> the server finishes
    srv.stop_streams.send(true).unwrap();
    tokio::time::timeout(Duration::from_secs(2), srv.handle)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn websocket_chat_between_two_clients() {
    let srv = start().await;
    let url = format!("ws://{}/ws", srv.addr);
    let (mut alice, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
    let (mut bob, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    alice
        .send(Message::text(r#"{"user":"alice","text":"hi bob"}"#))
        .await
        .unwrap();
    let got = tokio::time::timeout(Duration::from_secs(2), bob.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        got.into_text().unwrap().as_str(),
        r#"{"user":"alice","text":"hi bob"}"#
    );
}

#[tokio::test]
async fn graceful_shutdown_does_not_wait_for_websockets() {
    let mut srv = start().await;
    let url = format!("ws://{}/ws", srv.addr);
    let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    srv.stop_server.take().unwrap().send(()).unwrap();
    // an upgraded connection is no longer HTTP: `serve` stops tracking it and finishes
    tokio::time::timeout(Duration::from_secs(2), srv.handle)
        .await
        .unwrap()
        .unwrap();

    // ...while the socket task lives on in the runtime until the process exits
    let msg = ChatMessage {
        user: "server".into(),
        text: "still here".into(),
    };
    srv.hub.tx.send(msg).unwrap();
    let got = tokio::time::timeout(Duration::from_secs(2), ws.next())
        .await
        .unwrap();
    assert!(
        got.unwrap()
            .unwrap()
            .into_text()
            .unwrap()
            .contains("still here")
    );

    // the fix: flip the flag -> the socket task sends Close; the tracker sees it finish
    srv.stop_streams.send(true).unwrap();
    let got = tokio::time::timeout(Duration::from_secs(2), ws.next())
        .await
        .unwrap();
    assert!(matches!(got, Some(Ok(Message::Close(_)))), "{got:?}");
    srv.hub.sockets.close();
    tokio::time::timeout(Duration::from_secs(2), srv.hub.sockets.wait())
        .await
        .unwrap();
}

#[tokio::test]
async fn broadcast_send_without_receivers_is_err() {
    let (tx, rx) = tokio::sync::broadcast::channel::<u32>(4);
    drop(rx);
    assert!(tx.send(1).is_err());
}

#[tokio::test]
async fn slow_receiver_lags() {
    let (tx, mut rx) = tokio::sync::broadcast::channel::<u32>(2);
    for i in 0..5 {
        tx.send(i).unwrap();
    }
    let err = rx.recv().await.unwrap_err();
    println!("{err:?} / {err}");
    assert!(matches!(
        err,
        tokio::sync::broadcast::error::RecvError::Lagged(3)
    ));
    assert_eq!(rx.recv().await.unwrap(), 3);
}
