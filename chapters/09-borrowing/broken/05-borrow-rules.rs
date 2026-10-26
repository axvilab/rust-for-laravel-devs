//! Читач і редактор одночасно
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0502]: cannot borrow `list` as immutable because it is
//!   also borrowed as mutable
//! Виправлення: переставити рядки (останнє використання `c` — до `&list`) —
//!   див. examples/05-borrow-rules.rs
// expected-error: E0502

fn main() {
    let mut list = vec![1, 2, 3];
    let c = &mut list;
    c.push(4);
    let d = &list; // ПОМИЛКА: c ще використовується нижче
    c.push(5);
    println!("{d:?}");
}
