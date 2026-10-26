//! Вправа 3: `Excerpt` з посиланням на String, що знищився в кінці блоку
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0597]: `text` does not live long enough
//!   — borrowed value does not live long enough / `text` dropped here while still borrowed
//! Тобто: `e` тримає посилання на `text`, а `text` знищено на `}` раніше, ніж `e`
//! використано. Виправлення: examples/exercise-4.rs (Excerpt з String)
// expected-error: E0597

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let e;
    {
        let text = String::from("Перше речення. Друге.");
        e = Excerpt { part: &text };
    }
    println!("{}", e.part);
}
