//! Вправа 5: без `Debug` у `derive` вивід через `{:?}` не компілюється
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0277]: `Product` doesn't implement `Debug`
//!   note: add `#[derive(Debug)]` to `Product` or manually `impl Debug for Product`
//! Рядок `note:` — і є готова підказка. Виправлення: examples/exercise-4.rs
// expected-error: E0277

struct Product {
    id: u64,
    name: String,
}

fn main() {
    let p = Product {
        id: 1,
        name: "Ноутбук".into(),
    };
    println!("{p:#?}");
}
