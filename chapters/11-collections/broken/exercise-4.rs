//! Вправа 4 (зламана версія): використання `v` після `let v2 = v;`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `v`
//! Виправлення: `let v2 = v.clone();` — див. examples/exercise-4.rs
// expected-error: E0382

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("{v:?} {v2:?}");
}
