use orders_api::{
    MIGRATOR, app, config::Config, connect, init_tracing, shutdown_signal, state::AppState,
};

/// `anyhow::Result` in `main`: here we only need to fail with a readable
/// message (missing `JWT_SECRET`, database down). Domain errors use `thiserror`.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    init_tracing(config.log_json);
    tracing::info!(?config, "starting");

    let pool = connect(&config).await?;
    MIGRATOR.run(&pool).await?;

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "listening");

    axum::serve(listener, app(AppState::new(pool.clone(), config)))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    pool.close().await;
    tracing::info!("bye");
    Ok(())
}
