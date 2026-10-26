//! `/health` — the process is alive. `/ready` — it can serve (DB reachable).

use axum::extract::State;
use sqlx::PgPool;

use crate::error::AppError;

pub async fn health() -> &'static str {
    "ok"
}

/// Asks only for `State<PgPool>` thanks to `FromRef<AppState> for PgPool`.
pub async fn ready(State(db): State<PgPool>) -> Result<&'static str, AppError> {
    sqlx::query("SELECT 1").execute(&db).await?;
    Ok("ready")
}
