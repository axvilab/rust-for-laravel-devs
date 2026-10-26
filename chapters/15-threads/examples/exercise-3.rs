//! Exercise 3: the same with an `mpsc` channel

use std::sync::mpsc;
use std::thread;

fn main() {
    const N: u64 = 1_000_000;
    let (tx, rx) = mpsc::channel();

    for i in 0..4u64 {
        let tx = tx.clone();
        thread::spawn(move || {
            let part: u64 = (i * N / 4..(i + 1) * N / 4).sum();
            tx.send(part).unwrap(); // part moves into the channel
        });
    }
    drop(tx); // otherwise `rx.iter()` would wait forever

    let total: u64 = rx.iter().sum();
    println!("{total}"); // 499999500000
}
