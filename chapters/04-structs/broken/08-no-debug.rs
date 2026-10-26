//! `{:?}` for a struct without `#[derive(Debug)]`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: `User` doesn't implement `Debug`
//!   note: add `#[derive(Debug)]` to `User` or manually `impl Debug for User`
//! Fix: `#[derive(Debug)]` — see examples/08-derive-debug.rs
// expected-error: E0277

struct User {
    id: u64,
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Vitalii".into(),
    };
    println!("{:?}", user);
}
