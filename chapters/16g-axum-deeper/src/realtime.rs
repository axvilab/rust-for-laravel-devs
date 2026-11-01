//! Real time in one process: a `broadcast` channel, Server-Sent Events and WebSockets.

use std::{convert::Infallible, time::Duration};

use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{
        Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures_util::{SinkExt, Stream, StreamExt, stream::SplitSink};
use serde::{Deserialize, Serialize};
use tokio::sync::{
    broadcast::{self, error::RecvError},
    watch,
};
use tokio_stream::wrappers::{BroadcastStream, errors::BroadcastStreamRecvError};
use tokio_util::task::TaskTracker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: String,
    pub text: String,
}

/// The "event bus": every subscriber gets its own copy of every message.
#[derive(Clone)]
pub struct Hub {
    pub tx: broadcast::Sender<ChatMessage>,
    /// Flips to `true` on shutdown, so endless streams can finish.
    pub shutdown: watch::Receiver<bool>,
    /// WebSocket tasks: `axum::serve` does not wait for them, so we do.
    pub sockets: TaskTracker,
}

impl Hub {
    pub fn new(shutdown: watch::Receiver<bool>) -> Self {
        let (tx, _) = broadcast::channel(100); // a slow reader may lag 100 messages
        Self {
            tx,
            shutdown,
            sockets: TaskTracker::new(),
        }
    }
}

/// `event(new Message(...))` in Laravel; here — `send` to the channel.
async fn publish(State(hub): State<Hub>, Json(msg): Json<ChatMessage>) -> StatusCode {
    // Err only means "nobody is listening right now" — not a failure for us
    match hub.tx.send(msg) {
        Ok(listeners) => tracing::info!(listeners, "published"),
        Err(_) => tracing::info!("published to nobody"),
    }
    StatusCode::ACCEPTED
}

/// Resolves when the shutdown flag becomes `true`.
pub async fn wait_for_shutdown(mut rx: watch::Receiver<bool>) {
    let _ = rx.wait_for(|stop| *stop).await;
}

// --- Server-Sent Events -----------------------------------------------------------

async fn events(State(hub): State<Hub>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(hub.tx.subscribe())
        .map(|item| match item {
            Ok(msg) => Event::default().event("message").json_data(&msg),
            // the client was too slow and missed `n` messages: tell it
            Err(BroadcastStreamRecvError::Lagged(n)) => {
                Ok(Event::default().event("lagged").data(n.to_string()))
            }
        })
        .map(|ev| Ok(ev.unwrap_or_else(|_| Event::default().comment("bad event"))))
        // without this the stream never ends, and graceful shutdown waits forever
        .take_until(wait_for_shutdown(hub.shutdown.clone()));

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

// --- WebSocket --------------------------------------------------------------------

async fn ws(upgrade: WebSocketUpgrade, State(hub): State<Hub>) -> Response {
    // the HTTP handler only agrees to switch protocols; the work happens in `chat`
    let sockets = hub.sockets.clone();
    upgrade.on_upgrade(move |socket| sockets.track_future(chat(socket, hub)))
}

async fn chat(socket: WebSocket, hub: Hub) {
    let (to_client, mut from_client) = socket.split();
    let mut send_task = tokio::spawn(forward_bus(hub.clone(), to_client));

    // everything this client types -> the bus
    let tx = hub.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(frame)) = from_client.next().await {
            match frame {
                Message::Text(text) => {
                    if let Ok(msg) = serde_json::from_str::<ChatMessage>(text.as_str()) {
                        let _ = tx.send(msg);
                    }
                }
                Message::Close(_) => break,
                _ => {} // ping/pong are answered by the library, binary we ignore
            }
        }
    });

    // whichever finishes first, stop the other one too
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

/// Everything from the bus -> this client, until the client leaves or the server stops.
async fn forward_bus(hub: Hub, mut to_client: SplitSink<WebSocket, Message>) {
    let mut rx = hub.tx.subscribe();
    loop {
        tokio::select! {
            res = rx.recv() => match res {
                Ok(msg) => {
                    let json = serde_json::to_string(&msg).expect("ChatMessage serializes");
                    if to_client.send(Message::Text(json.into())).await.is_err() {
                        return; // the client is gone
                    }
                }
                Err(RecvError::Lagged(n)) => tracing::warn!(n, "client lagged"), // keep going
                Err(RecvError::Closed) => return,
            },
            _ = wait_for_shutdown(hub.shutdown.clone()) => {
                let _ = to_client.send(Message::Close(None)).await; // a polite goodbye
                return;
            }
        }
    }
}

pub fn app(hub: Hub) -> Router {
    Router::new()
        .route("/messages", post(publish))
        .route("/events", get(events))
        .route("/ws", get(ws))
        .with_state(hub)
}
