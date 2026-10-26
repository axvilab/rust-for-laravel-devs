//! `thread::scope` — lend local data to threads without `Arc` and `move`

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    thread::scope(|s| {
        let (left, right) = data.split_at(3);
        s.spawn(|| println!("left sum: {}", left.iter().sum::<i32>()));
        s.spawn(|| println!("right sum: {}", right.iter().sum::<i32>()));
    }); // scope waits for all the threads here

    println!("{data:?}"); // data is still available
}
