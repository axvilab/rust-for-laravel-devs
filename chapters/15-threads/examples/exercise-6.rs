//! Exercise 6: a deadlock from a different lock order — and the fix with one order
//!
//! Thread A takes 1, then 2; thread B takes 2, then 1. If each managed to take its
//! first lock, both wait forever. So that the example doesn't hang, the main thread
//! waits for the result with a timeout. Rust saves you from data races, not deadlocks.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn transfer(first: &Mutex<i32>, second: &Mutex<i32>) {
    let mut a = first.lock().unwrap();
    thread::sleep(Duration::from_millis(50)); // let the other thread take its lock
    let mut b = second.lock().unwrap();
    *a -= 1;
    *b += 1;
}

fn run(same_order: bool) -> bool {
    let m1 = Arc::new(Mutex::new(100));
    let m2 = Arc::new(Mutex::new(100));
    let (done_tx, done_rx) = mpsc::channel();

    for thread_b in [false, true] {
        let (m1, m2, done) = (Arc::clone(&m1), Arc::clone(&m2), done_tx.clone());
        thread::spawn(move || {
            if thread_b && !same_order {
                transfer(&m2, &m1); // B: 2, then 1 — the opposite order
            } else {
                transfer(&m1, &m2); // A (and B in the fixed version): 1, then 2
            }
            let _ = done.send(());
        });
    }
    // both threads must report within a second
    (0..2).all(|_| done_rx.recv_timeout(Duration::from_secs(1)).is_ok())
}

fn main() {
    println!("different lock order: finished = {}", run(false)); // false — deadlock
    println!("same order:           finished = {}", run(true)); // true
    // The deadlocked threads stay stuck; the process ends together with main.
}
