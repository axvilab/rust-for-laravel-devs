//! `thread::spawn` — start a thread; `join` — wait for it and take the result

use std::thread;

fn heavy_computation() -> u64 {
    (1..=10_000_000u64).sum()
}

fn main() {
    let handle = thread::spawn(|| {
        // this runs in a separate thread
        heavy_computation()
    });

    // meanwhile the main thread does its own thing...
    println!("the main thread keeps working");

    let result = handle.join().unwrap(); // wait and take the result
    println!("{result}");
}
