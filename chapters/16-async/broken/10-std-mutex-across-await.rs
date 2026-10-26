//! `std::sync::MutexGuard` живий під час `.await` — future перестає бути `Send`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано. (Без tokio: `require_send` перевіряє те саме,
//! що `tokio::spawn` — що future можна передати в інший потік.)
//! Очікувана помилка: error: future cannot be sent between threads safely
//!   note: future is not `Send` as this value is used across an await
//! Виправлення: закрити замок блоком до `.await` або tokio::sync::Mutex —
//!   див. examples/09-mutex-and-await.rs
// expected-error: future cannot be sent between threads safely

use std::sync::Mutex;

async fn save() {}

fn require_send<T: Send>(_: T) {}

fn main() {
    let state = Mutex::new(vec![1u32]);
    require_send(async {
        let mut guard = state.lock().unwrap();
        save().await;
        guard.push(2);
    });
}
