//! Вправа 5: `checked_add` і `saturating_add` для u8
//!
//! `stock + 10` напряму компілятор відхиляє, бо бачить переповнення вже під час
//! збірки: `error: this arithmetic operation will overflow` (див. broken/exercise-5.rs).

fn main() {
    let stock: u8 = 250;
    println!("{:?}", stock.checked_add(10)); // None — 260 не влазить в u8
    println!("{:?}", stock.saturating_add(10)); // 255 — притиснуло до максимуму
    println!("{:?}", stock.checked_add(5)); // Some(255) — влазить
}
