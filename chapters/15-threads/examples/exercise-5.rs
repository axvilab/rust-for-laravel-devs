//! Вправа 5: `match` по `lock()` тримає замок до кінця match — `try_lock` це показує
//!
//! Надрукується «вільний = false» у першому випадку (guard живе до кінця match;
//! `lock()` замість `try_lock()` тут був би дедлок — потік чекав би сам на себе)
//! і «вільний = true» після переписування через `let value = ...cloned();`.

use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let map = Mutex::new(HashMap::from([(1, "один".to_string())]));

    match map.lock().unwrap().get(&1) {
        Some(v) => println!("{v}: вільний = {}", map.try_lock().is_ok()),
        None => println!("нема"),
    }

    let value = map.lock().unwrap().get(&1).cloned(); // guard знищено в кінці рядка
    match value {
        Some(v) => println!("{v}: вільний = {}", map.try_lock().is_ok()),
        None => println!("нема"),
    }
}
