//! The trap: `drop(guard)` before `.await` does NOT help (still true on Rust 1.97)
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error: future cannot be sent between threads safely
//!   (the compiler looks at scopes, not at drop calls)
//! Fix: close the guard in a block `{ ... }` — see examples/09-mutex-and-await.rs
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
