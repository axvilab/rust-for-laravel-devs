//! Те саме, але `first` використано ПІСЛЯ `push`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0502]: cannot borrow `scores` as mutable because it is
//!   also borrowed as immutable
//! Виправлення: використати посилання раніше — див. examples/07-nll.rs
// expected-error: E0502

fn main() {
    let mut scores = vec![10, 20];
    let first = &scores[0];
    scores.push(30);
    println!("{first}");
}
