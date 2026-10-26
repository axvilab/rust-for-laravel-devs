//! `let b = a` для String — переїзд: `a` більше не можна використовувати
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `a`
//! Виправлення: користуватися `b`, або `let b = a.clone();` — див. examples/02-move.rs
// expected-error: E0382

fn main() {
    let a = String::from("привіт");
    let b = a; // володіння ПЕРЕЙШЛО до b
    println!("{a}"); // ПОМИЛКА: borrow of moved value: `a`
    println!("{b}");
}
