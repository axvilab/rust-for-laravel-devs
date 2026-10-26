//! Exercise 1: 4 threads sum parts of `0..1_000_000`, the results come via `join`

use std::thread;
use std::time::Instant;

fn main() {
    const N: u64 = 1_000_000;
    const PARTS: u64 = 4;

    let start = Instant::now();
    let handles: Vec<_> = (0..PARTS)
        .map(|i| {
            let (from, to) = (i * N / PARTS, (i + 1) * N / PARTS);
            thread::spawn(move || (from..to).sum::<u64>()) // move: from/to — Copy
        })
        .collect();
    let parallel: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    let t_parallel = start.elapsed();

    let start = Instant::now();
    let sequential: u64 = (0..N).sum();
    let t_seq = start.elapsed();

    assert_eq!(parallel, sequential);
    println!("sum {parallel}: threads {t_parallel:?}, sequential {t_seq:?}");
    // For such tiny work sequential is often faster: creating a thread
    // costs more than adding a million numbers (especially in release).
}
