//! Пастка: `drop(guard)` перед `.await` НЕ допомагає (на Rust 1.97 — так само)
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error: future cannot be sent between threads safely
//!   (компілятор дивиться на області видимості, а не на виклики drop)
//! Виправлення: закрити guard блоком `{ ... }` — див. examples/09-mutex-and-await.rs
// expected-error: future cannot be sent between threads safely

use std::sync::Mutex;

async fn save() {}

fn require_send<T: Send>(_: T) {}

fn main() {
    let state = Mutex::new(vec![1u32]);
    require_send(async {
        let mut guard = state.lock().unwrap();
        guard.push(2);
        drop(guard);
        save().await;
    });
}
