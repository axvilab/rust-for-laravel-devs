//! Два `&mut` одночасно: змінювати в аргументах того ж виклику не можна
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0499]: cannot borrow `v` as mutable more than once at a time
//! Виправлення: `let last = v.pop().unwrap(); v.push(last);` — див. examples/08-two-phase-borrows.rs
// expected-error: E0499

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.pop().unwrap());
    println!("{v:?}");
}
