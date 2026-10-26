//! Exercise 5: `match` on `lock()` holds the lock until the end of the match — `try_lock` shows it
//!
//! It prints "free = false" in the first case (the guard lives until the end of the match;
//! `lock()` instead of `try_lock()` would deadlock here — the thread would wait for itself)
//! and "free = true" after rewriting with `let value = ...cloned();`.

use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let map = Mutex::new(HashMap::from([(1, "one".to_string())]));

    match map.lock().unwrap().get(&1) {
        Some(v) => println!("{v}: free = {}", map.try_lock().is_ok()),
        None => println!("none"),
    }

    let value = map.lock().unwrap().get(&1).cloned(); // the guard is destroyed at the end of the line
    match value {
        Some(v) => println!("{v}: free = {}", map.try_lock().is_ok()),
        None => println!("none"),
    }
}
