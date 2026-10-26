//! Тимчасовий guard у `match` живе до кінця всього `match`

use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let cache = Mutex::new(HashMap::from([(1, String::from("один"))]));

    // clippy радить тут `if let` (single_match); залишаємо match, як у книзі —
    // пастка однакова в обох формах: guard живе до кінця блоку.
    #[allow(clippy::single_match)]
    match cache.lock().unwrap().get(&1) {
        Some(v) => {
            // замок ДОСІ взятий: тимчасовий guard живе до кінця всього match
            // cache.lock() тут — дедлок; try_lock показує, що замок зайнятий:
            println!("{v}: вільний = {}", cache.try_lock().is_ok()); // false
        }
        None => {}
    }

    // безпечно: guard знищиться в кінці рядка, у змінній — копія
    let value = cache.lock().unwrap().get(&1).cloned();
    if let Some(v) = value {
        println!("{v}: вільний = {}", cache.try_lock().is_ok()); // true
    }
}
