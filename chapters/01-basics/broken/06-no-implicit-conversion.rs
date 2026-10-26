//! Немає неявних перетворень: i32 + f64 не компілюється
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: cannot add `f64` to `i32`
//! Виправлення: явне перетворення `x as f64 + y` — див. examples/06-as-cast.rs
// expected-error: E0277

fn main() {
    let x: i32 = 5;
    let y: f64 = 2.5;
    let z = x + y; // ПОМИЛКА: cannot add f64 to i32
    println!("{z}");
}
