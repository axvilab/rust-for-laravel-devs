//! A minimal Axum app: State, Path, Json, Result, the route `/users/{id}`
// long-running: a server on http://localhost:3000, stop it with Ctrl+C
//   curl -s localhost:3000/users/1    → {"id":1,"name":"Vitalii"}
//   curl -si localhost:3000/users/9   → 404 {"error":"not found"}
//
// In the book the handler reads Postgres via sqlx::query_as!. Here the state is an
// in-memory HashMap so the example runs without a database; the full sqlx version is in orders-api/.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde_json::{Value, json};

#[derive(Clone)]
struct AppState {
    db: Arc<HashMap<u64, String>>, // instead of sqlx::PgPool; Arc — a cheap Clone
}

enum AppError {
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))).into_response()
            }
        }
    }
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Value>, AppError> {
    let name = state.db.get(&id).ok_or(AppError::NotFound)?;
    Ok(Json(json!({ "id": id, "name": name })))
}

#[tokio::main]
async fn main() {
    let db = Arc::new(HashMap::from([(1, "Vitalii".to_string())]));
    let app = Router::new()
        .route("/users/{id}", get(get_user)) // Axum 0.8: {id}, not :id
        .with_state(AppState { db });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
