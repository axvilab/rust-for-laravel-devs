//! `Arc<Mutex<T>>`: кілька власників + змінювати по черзі

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ще один власник (тільки лічильник +1)
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().unwrap(); // взяти замок
            *guard += 1;
        })); // замок відпускається тут (drop guard)
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap()); // 10
}
