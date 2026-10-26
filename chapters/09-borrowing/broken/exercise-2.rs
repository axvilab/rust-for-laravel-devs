//! Вправа 2 (зламана версія): `&mut` на незмінну змінну
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
//! Виправлення: `let mut s` — див. examples/exercise-2.rs
// expected-error: E0596

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let s = String::from("Привіт");
    append_exclamation(&mut s);
    println!("{s}");
}
