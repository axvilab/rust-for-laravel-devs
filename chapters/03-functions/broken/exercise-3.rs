//! Вправа 3 (зламана версія): `String` після `move`-замикання
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `label`
//!   з поясненням `value moved into closure here` / `variable moved due to use in closure`
//! Виправлення: див. examples/exercise-3.rs (або `label.clone()` для замикання)
// expected-error: E0382

fn main() {
    let label = String::from("знижка");
    let show = move || println!("{label}");
    show();
    println!("{label}");
}
