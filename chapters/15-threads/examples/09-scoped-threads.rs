//! `thread::scope` — позичити локальні дані потокам без `Arc` і `move`

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    thread::scope(|s| {
        let (left, right) = data.split_at(3);
        s.spawn(|| println!("сума лівої: {}", left.iter().sum::<i32>()));
        s.spawn(|| println!("сума правої: {}", right.iter().sum::<i32>()));
    }); // scope чекає всі потоки тут

    println!("{data:?}"); // data все ще доступний
}
