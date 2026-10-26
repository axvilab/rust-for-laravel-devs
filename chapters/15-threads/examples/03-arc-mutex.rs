//! `Arc<Mutex<T>>`: several owners + change in turn

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // one more owner (just counter +1)
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap(); // take the lock
            *guard += 1;
        })); // the lock is released here (drop guard)
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap()); // 10
}
