//! Chapter 14. Modules, crates and Cargo — how to organize a project
//!
//! This crate is itself an example for the chapter: a library (`lib.rs` + modules) with the logic,
//! a thin `main.rs`, a separate command in `src/bin/`, doc tests.
//!
//! ```text
//! src/
//! ├── lib.rs           ← the library root: declares the modules
//! ├── main.rs          ← the binary: a few lines, calls the library
//! ├── bin/report.rs    ← one more command: cargo run -p ch14-modules --bin report
//! ├── models.rs        ← Product, Order
//! ├── pricing.rs       ← calculations
//! ├── errors.rs        ← ShopError
//! ├── services.rs      ← declares the submodules + pub use
//! └── services/
//!     ├── payment.rs   ← services::payment
//!     └── mail.rs      ← services::mail
//! ```
//!
//! The module layout "as in the book" with `mod` in `main.rs` is the example
//! `examples/01-modules/` (`cargo run -p ch14-modules --example 01-modules`).

pub mod errors;
pub mod models;
pub mod pricing;
pub mod services;

/// Calculates the fee for a payment: 2% (the amount is in kopecks).
///
/// A doc test is a separate program that uses the crate FROM OUTSIDE, so `use` is needed:
///
/// ```
/// use ch14_modules::calc_fee;
///
/// assert_eq!(calc_fee(1000), 20);
/// ```
///
/// Without `use` the same example doesn't compile — `compile_fail` checks exactly that
/// (error[E0425]: cannot find function `calc_fee` in this scope):
///
/// ```compile_fail,E0425
/// let fee = calc_fee(1000);
/// assert_eq!(fee, 20);
/// ```
pub fn calc_fee(amount: u64) -> u64 {
    amount / 50
}

/// The application entry point: `main.rs` only calls it.
pub fn run() -> Result<(), errors::ShopError> {
    use models::{Order, Product};
    use services::{Mailer, PaymentService};

    let laptop = Product::new(1, "Laptop", 4_500_000);
    let order = Order::new(7, vec![(laptop, 1)])?;
    let total = pricing::total_with_fee(&order);

    PaymentService::new().charge("v@example.com", total)?;
    Mailer::new().send("v@example.com", &format!("Order #{} paid", order.id));
    Ok(())
}
