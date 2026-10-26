//! Вправа 3: посилання на елемент, потім `push`, потім читання посилання
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0502]: cannot borrow `v` as mutable because it is
//!   also borrowed as immutable
//! Чому: `push` може перевиділити пам'ять вектора (не вистачило місця — дані
//! переїжджають у новий, більший блок), і `first` вказував би на звільнене місце.
//! Виправлення: examples/exercise-4.rs
// expected-error: E0502

fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{first}");
}
