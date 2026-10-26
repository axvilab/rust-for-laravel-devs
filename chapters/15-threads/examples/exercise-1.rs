//! Вправа 1: 4 потоки рахують суми частин `0..1_000_000`, результати — через `join`

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
    println!("сума {parallel}: потоки {t_parallel:?}, послідовно {t_seq:?}");
    // Для такої дрібної роботи послідовно часто швидше: створення потоку
    // коштує більше, ніж додати мільйон чисел (особливо в release).
}
