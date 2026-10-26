//! Змінити значення через `&` не можна
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0594]: cannot assign to `*s`, which is behind a `&` reference
//! Виправлення: `&mut` у посиланні і `let mut` у власника — див. examples/04-mut-binding-vs-mut-ref.rs
// expected-error: E0594

fn main() {
    let b = 2;
    let s = &b;
    *s += 1;
    println!("{s}");
}
