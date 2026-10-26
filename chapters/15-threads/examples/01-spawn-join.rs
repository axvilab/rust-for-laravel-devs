//! `thread::spawn` — запустити потік; `join` — дочекатися і забрати результат

use std::thread;

fn heavy_computation() -> u64 {
    (1..=10_000_000u64).sum()
}

fn main() {
    let handle = thread::spawn(|| {
        // це виконується в окремому потоці
        heavy_computation()
    });

    // тут головний потік робить своє...
    println!("головний потік працює далі");

    let result = handle.join().unwrap(); // дочекатися і забрати результат
    println!("{result}");
}
