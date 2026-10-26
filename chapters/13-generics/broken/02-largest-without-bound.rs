//! Generic без обмеження: компілятор не знає, чи можна порівнювати `T`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0369]: binary operation `>` cannot be applied to type `&T`
//!   help: consider restricting type parameter `T` with trait `PartialOrd`
//! Виправлення: `fn largest<T: PartialOrd>` — див. examples/02-largest.rs
// expected-error: E0369

fn largest<T>(items: &[T]) -> &T {
    let mut max = &items[0];
    for item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 7, 2]));
}
