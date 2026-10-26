//! Вправа 5: `Copy` для структури з полем `String`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0204]: the trait `Copy` cannot be implemented for this type
//!   (this field does not implement `Copy`)
//! Чому: String тримає дані в купі; побітова копія дала б двох власників
//! одного шматка пам'яті. Виправлення: тільки `Clone` — див. examples/exercise-5.rs
// expected-error: E0204

#[derive(Clone, Copy)]
struct User {
    id: u64,
    name: String,
}

fn main() {}
