//! Попередження компілятора: невикористана змінна і `_`-префікс
//!
//! Цей файл містить `#![allow(unused_variables)]`, бо CI збирає з `-D warnings`.
//! Прибери рядок з `allow` і запусти `cargo build -p ch01-basics --example 03-warnings` —
//! побачиш `warning: unused variable: x` і підказку `help: ... prefix it with an underscore`.
#![allow(unused_variables)]

fn main() {
    let x = 5; // warning: unused variable: `x`
    let _y = 6; // тиша: «знаю, що не використовую»
}
