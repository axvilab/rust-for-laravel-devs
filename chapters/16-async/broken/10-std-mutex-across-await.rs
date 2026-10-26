//! A `std::sync::MutexGuard` alive across `.await` — the future is no longer `Send`
//!
//! DOES NOT COMPILE — on purpose. (Without tokio: `require_send` checks the same thing
//! as `tokio::spawn` — that the future can be sent to another thread.)
//! Expected error: error: future cannot be sent between threads safely
//!   note: future is not `Send` as this value is used across an await
//! Fix: close the lock in a block before `.await`, or use tokio::sync::Mutex —
//!   see examples/09-mutex-and-await.rs
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
