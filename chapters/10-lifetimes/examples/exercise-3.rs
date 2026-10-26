//! Exercise 3: `Excerpt<'a>` works as long as the source lives longer
//!
//! The version where `text` is created inside a block and `Excerpt` is used after it
//! gives E0597 `text does not live long enough` (broken/exercise-3.rs).
//! Here the source is declared outside — and everything compiles.

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let text = String::from("First sentence. Second.");
    let e;
    {
        let first = text.split('.').next().unwrap_or("");
        e = Excerpt { part: first };
    }
    println!("{}", e.part); // ok: text is alive
}
