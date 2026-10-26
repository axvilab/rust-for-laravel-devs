//! Тримай замок коротко: повільну роботу — до `lock()`

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn slow_network_call(i: u64) -> u64 {
    thread::sleep(Duration::from_millis(50));
    i
}

fn run(hold_lock_during_call: bool) -> Duration {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                if hold_lock_during_call {
                    // погано: замок тримається весь час, поки працює повільна функція
                    let mut data = shared.lock().unwrap();
                    let result = slow_network_call(i);
                    data.push(result);
                } else {
                    // добре: взяли, записали, відпустили
                    let result = slow_network_call(i);
                    shared.lock().unwrap().push(result);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    start.elapsed()
}

fn main() {
    println!("замок через виклик: {:?}", run(true)); // ~200 мс: потоки по черзі
    println!("короткий замок:     {:?}", run(false)); // ~50 мс: паралельно
}
