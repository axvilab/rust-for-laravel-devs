//! Вправа 3: те саме через канал `mpsc`

use std::sync::mpsc;
use std::thread;

fn main() {
    const N: u64 = 1_000_000;
    let (tx, rx) = mpsc::channel();

    for i in 0..4u64 {
        let tx = tx.clone();
        thread::spawn(move || {
            let part: u64 = (i * N / 4..(i + 1) * N / 4).sum();
            tx.send(part).unwrap(); // part переїжджає в канал
        });
    }
    drop(tx); // інакше `rx.iter()` чекав би вічно

    let total: u64 = rx.iter().sum();
    println!("{total}"); // 499999500000
}
