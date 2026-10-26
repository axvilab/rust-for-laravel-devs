//! Attributes: `#![allow]` for the whole file and `#[cfg]` — conditional compilation
//!
//! `cargo run -p ch04-structs --example 07-attributes-cfg`           → build: debug
//! `cargo run -p ch04-structs --example 07-attributes-cfg --release` → build: release
#![allow(dead_code)] // inner: for the whole file — don't warn about unused code

struct Draft {
    title: String,
} // used nowhere — and silence

#[cfg(debug_assertions)] // this version exists only in a debug build
fn build_mode() -> &'static str {
    "debug"
}

#[cfg(not(debug_assertions))] // and this one only in release
fn build_mode() -> &'static str {
    "release"
}

fn main() {
    println!("build: {}", build_mode());
}
