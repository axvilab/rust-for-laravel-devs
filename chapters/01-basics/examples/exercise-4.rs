//! Вправа 4: `dbg!` посеред виразу і порядок виводу
//!
//! `dbg!` пише в stderr і повертає значення далі, тому решта рядка не змінюється.
//! Спершу з'явиться `[examples/exercise-4.rs:..] price * qty as f64 = 599.97`,
//! потім — рядок від `println!`. У терміналі обидва потоки змішуються, але
//! `cargo run ... 2>/dev/null` залишить тільки вивід `println!`.

fn main() {
    let price: f64 = 199.99;
    let qty: u32 = 3;
    let total = dbg!(price * qty as f64);
    println!("сума: {total:.2}");
}
