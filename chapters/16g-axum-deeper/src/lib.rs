//! Chapter Axum, deeper: layers, cookies, files and real time
//!
//! Each topic is a module with functions that build a `Router`, so the same router
//! is served by `examples/` and checked by `tests/`.

pub mod cookies;
pub mod extractors;
pub mod files;
pub mod layers;
pub mod limits;
pub mod pg_store;
pub mod realtime;
pub mod sessions;
pub mod uploads;

use std::net::SocketAddr;

use axum::Router;

/// Logs to the console (`RUST_LOG` works) and serves `app` on 127.0.0.1:3000.
/// `ConnectInfo<SocketAddr>` is enabled: the rate limiter needs the client's address.
pub async fn serve(app: Router) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=info".into());
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
