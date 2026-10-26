//! Вправа 3 (зламана версія): рядок використано після передачі у `shout`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `s`
//! Виправлення: див. examples/exercise-3.rs
// expected-error: E0382

fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let s = String::from("привіт");
    let loud = shout(s);
    println!("{s} → {loud}");
}
