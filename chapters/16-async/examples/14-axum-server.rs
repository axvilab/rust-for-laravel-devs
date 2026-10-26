//! Мінімальний Axum: State, Path, Json, Result, роут `/users/{id}`
// long-running: сервер на http://localhost:3000, зупинити — Ctrl+C
//   curl -s localhost:3000/users/1    → {"id":1,"name":"Віталій"}
//   curl -si localhost:3000/users/9   → 404 {"error":"not found"}
//
// У книзі хендлер читає з Postgres через sqlx::query_as!. Тут стан — HashMap у
// пам'яті, щоб приклад запускався без бази; повна версія з sqlx — у orders-api/.

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
    db: Arc<HashMap<u64, String>>, // замість sqlx::PgPool; Arc — дешевий Clone
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
    let db = Arc::new(HashMap::from([(1, "Віталій".to_string())]));
    let app = Router::new()
        .route("/users/{id}", get(get_user)) // Axum 0.8: {id}, не :id
        .with_state(AppState { db });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("слухаю :3000");
    axum::serve(listener, app).await.unwrap();
}
