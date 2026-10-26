//! Атрибути: `#![allow]` на весь файл і `#[cfg]` — умовна компіляція
//!
//! `cargo run -p ch04-structs --example 07-attributes-cfg`           → збірка: debug
//! `cargo run -p ch04-structs --example 07-attributes-cfg --release` → збірка: release
#![allow(dead_code)] // внутрішній: на весь файл — не попереджай про невикористане

struct Draft {
    title: String,
} // ніде не використовується — і тиша

#[cfg(debug_assertions)] // ця версія існує тільки в debug-збірці
fn build_mode() -> &'static str {
    "debug"
}

#[cfg(not(debug_assertions))] // а ця — тільки в release
fn build_mode() -> &'static str {
    "release"
}

fn main() {
    println!("збірка: {}", build_mode());
}
