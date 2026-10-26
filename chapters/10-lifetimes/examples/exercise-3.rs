//! Вправа 3: `Excerpt<'a>` працює, поки джерело живе довше
//!
//! Версія, де `text` створено в блоці, а `Excerpt` використано після нього,
//! дає E0597 `text does not live long enough` (broken/exercise-3.rs).
//! Тут джерело оголошено зовні — і все компілюється.

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let text = String::from("Перше речення. Друге.");
    let e;
    {
        let first = text.split('.').next().unwrap_or("");
        e = Excerpt { part: first };
    }
    println!("{}", e.part); // ok: text живий
}
