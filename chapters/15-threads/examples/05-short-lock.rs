//! Keep the lock short: do slow work before `lock()`

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
                    // bad: the lock is held the whole time the slow function runs
                    let mut data = shared.lock().unwrap();
                    let result = slow_network_call(i);
                    data.push(result);
                } else {
                    // good: took it, wrote, released
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
    println!("lock held through the call: {:?}", run(true)); // ~200 ms: threads take turns
    println!("short lock:                 {:?}", run(false)); // ~50 ms: in parallel
}
