//! Вправа 3 (зламана версія): `Option<Reply>` усередині `Reply`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0072]: recursive type `Reply` has infinite size
//!   help: insert some indirection … `parent: Option<Box<Reply>>`
//! Виправлення: examples/exercise-3.rs
// expected-error: E0072

struct Reply {
    text: String,
    parent: Option<Reply>,
}

fn main() {}
