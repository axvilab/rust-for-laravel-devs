//! `?` in a plain `main` that returns nothing
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: the `?` operator can only be used in a function
//!   that returns `Result` or `Option`
//! Fix: `fn main() -> Result<(), ParseIntError>` — see examples/04-main-result.rs
// expected-error: E0277

fn main() {
    let port: u16 = "80a".parse()?;
    println!("port {port}");
}
