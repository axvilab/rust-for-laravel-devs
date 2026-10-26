//! A temporary guard in `match` lives until the end of the whole `match`

use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let cache = Mutex::new(HashMap::from([(1, String::from("one"))]));

    // clippy suggests `if let` here (single_match); we keep match as in the book —
    // the trap is the same in both forms: the guard lives until the end of the block.
    #[allow(clippy::single_match)]
    match cache.lock().unwrap().get(&1) {
        Some(v) => {
            // the lock is STILL held: the temporary guard lives until the end of the whole match
            // cache.lock() here would deadlock; try_lock shows that the lock is taken:
            println!("{v}: free = {}", cache.try_lock().is_ok()); // false
        }
        None => {}
    }

    // safe: the guard is destroyed at the end of the line, the variable holds a copy
    let value = cache.lock().unwrap().get(&1).cloned();
    if let Some(v) = value {
        println!("{v}: free = {}", cache.try_lock().is_ok()); // true
    }
}
