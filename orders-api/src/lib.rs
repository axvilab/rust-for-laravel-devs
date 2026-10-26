//! orders-api — the end-to-end project from the book
//! "Rust для Laravel-розробника" (chapters 16b–16f).
//!
//! Everything lives in the library so integration tests in `tests/` can build
//! the same `Router` that `main.rs` serves.

pub mod auth;
pub mod config;
pub mod domain;
pub mod error;
pub mod filter;
pub mod handlers;
pub mod middleware;
pub mod pagination;
pub mod repo;
pub mod state;
pub mod validation;

use std::time::Duration;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::{
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::{
    config::Config,
    error::AppError,
    handlers::{auth as auth_h, health, orders},
    state::AppState,
};

/// Embedded into the binary at compile time; no `migrations/` dir needed at runtime.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// The whole HTTP surface. `.layer()` wraps everything declared above it;
/// read layers bottom-up: `request_id` is the outermost, `Compression` is the
/// closest to the handler.
pub fn app(state: AppState) -> Router {
    let api_v1 = Router::new()
        .route("/auth/register", post(auth_h::register))
        .route("/auth/login", post(auth_h::login))
        .route("/me", get(auth_h::me))
        .route("/orders", post(orders::create).get(orders::list))
        .route("/orders/{id}", get(orders::show))
        .route("/orders/{id}/items", post(orders::add_item))
        .route("/orders/{id}/pay", post(orders::pay))
        .route("/orders/{id}/cancel", post(orders::cancel))
        .route("/admin/orders", get(orders::admin_list));

    let timeout = Duration::from_secs(state.config.request_timeout_secs);
    let cors = middleware::cors(&state.config.cors_origins());

    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .nest("/api/v1", api_v1)
        .fallback(|| async { AppError::NotFound })
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            timeout,
        ))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(middleware::make_span)
                // default is DEBUG; INFO logs status + latency of every request
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(axum::middleware::from_fn(middleware::request_id))
        .with_state(state)
}

pub async fn connect(config: &Config) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await?;
    Ok(pool)
}

pub fn init_tracing(json: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,sqlx=warn,tower_http=info".into());
    let builder = tracing_subscriber::fmt().with_env_filter(filter);
    if json {
        builder.json().init(); // Loki / CloudWatch / Datadog
    } else {
        builder.init(); // human-readable
    }
}

/// Resolves on Ctrl+C or SIGTERM (`docker stop`, Kubernetes).
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received, draining connections");
}
