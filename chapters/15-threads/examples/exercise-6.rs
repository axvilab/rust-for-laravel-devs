//! Вправа 6: дедлок через різний порядок замків — і виправлення одним порядком
//!
//! Потік А бере 1, потім 2; потік Б — 2, потім 1. Якщо кожен встиг узяти свій
//! перший замок, обидва чекають вічно. Щоб приклад не висів, головний потік
//! чекає результат із таймаутом. Rust рятує від гонок даних, не від дедлоків.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn transfer(first: &Mutex<i32>, second: &Mutex<i32>) {
    let mut a = first.lock().unwrap();
    thread::sleep(Duration::from_millis(50)); // даємо іншому потоку взяти свій замок
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
                transfer(&m2, &m1); // Б: 2, потім 1 — протилежний порядок
            } else {
                transfer(&m1, &m2); // А (і Б у виправленій версії): 1, потім 2
            }
            let _ = done.send(());
        });
    }
    // обидва потоки мають відзвітувати протягом секунди
    (0..2).all(|_| done_rx.recv_timeout(Duration::from_secs(1)).is_ok())
}

fn main() {
    println!("різний порядок замків: завершилось = {}", run(false)); // false — дедлок
    println!("однаковий порядок:     завершилось = {}", run(true)); // true
    // Потоки з дедлоку так і висять; процес завершується разом із main.
}
