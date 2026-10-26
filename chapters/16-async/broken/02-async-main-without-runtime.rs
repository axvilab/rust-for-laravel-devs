//! `async fn main` without `#[tokio::main]`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0752]: `main` function is not allowed to be `async`
//! Fix: `#[tokio::main]` above `async fn main` — see examples/02-tokio-main.rs
// expected-error: E0752

async fn main() {
    println!("hello");
}
