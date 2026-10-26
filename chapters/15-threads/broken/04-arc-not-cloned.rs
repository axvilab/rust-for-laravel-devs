//! Забули `Arc::clone` перед `move` у циклі
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: the type `Arc` does not implement `Copy`
//!   value moved into closure here, in previous iteration of loop
//!   this move could be avoided by cloning the original `Arc`, which is inexpensive
//! Виправлення: `let counter = Arc::clone(&counter);` у тілі циклу — examples/03-arc-mutex.rs
// expected-error: does not implement `Copy`

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
