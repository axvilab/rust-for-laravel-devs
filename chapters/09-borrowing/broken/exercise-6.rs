//! Вправа 6 (зламана версія): `v.push(v.pop().unwrap())`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0499]: cannot borrow `v` as mutable more than once at a time
//! Виправлення: тимчасова змінна — див. examples/exercise-6.rs
// expected-error: E0499

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.pop().unwrap());
    println!("{v:?}");
}
