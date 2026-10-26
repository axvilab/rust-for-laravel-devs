//! `#[axum::debug_handler]` — людські помилки для неправильних сигнатур хендлерів
//!
//! Якщо поставити `Json` ПЕРЕД `Path`, звичайна помилка — `the trait bound ... Handler<_, _>
//! is not satisfied`. З атрибутом: «Json<_> consumes the request body and thus must be
//! the last argument to the handler function». Тут порядок уже правильний.
//! (Потрібна feature `macros`: `cargo add axum --features macros`.)

use axum::{Json, Router, extract::Path, routing::post};

#[axum::debug_handler]
async fn hello(Path(id): Path<u64>, Json(body): Json<serde_json::Value>) -> String {
    format!("{id}: {body}")
}

fn main() {
    // Лише збираємо роутер: приклад про компіляцію, а не про запуск сервера.
    let _app: Router = Router::new().route("/hello/{id}", post(hello));
    println!("хендлер має правильну сигнатуру — компілюється");
}
