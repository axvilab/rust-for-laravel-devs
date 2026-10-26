//! Exercise 2: the same with a shared `Arc<Mutex<u64>>`

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const N: u64 = 1_000_000;
    let total = Arc::new(Mutex::new(0u64));

    let handles: Vec<_> = (0..4u64)
        .map(|i| {
            let total = Arc::clone(&total);
            thread::spawn(move || {
                let part: u64 = (i * N / 4..(i + 1) * N / 4).sum(); // computed WITHOUT the lock
                *total.lock().unwrap() += part; // under the lock — only the addition
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *total.lock().unwrap()); // 499999500000
}
