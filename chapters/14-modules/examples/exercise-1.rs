//! Exercise 1: a "shop" split into modules — models, pricing, errors (see this crate's src/)
//!
//! The solution is the library itself: src/models.rs (Product, Order), src/pricing.rs
//! (calculations), src/errors.rs (ShopError), included in src/lib.rs with `pub mod`.
//! Here all three modules are used.

use ch14_modules::errors::ShopError;
use ch14_modules::models::{Order, Product};
use ch14_modules::pricing;

fn main() {
    let order = Order::new(
        42,
        vec![
            (Product::new(1, "Laptop", 4_500_000), 1),
            (Product::new(2, "Mouse", 80_000), 2),
        ],
    )
    .unwrap();
    println!(
        "#{}: {}",
        order.id,
        pricing::format_uah(pricing::total_with_fee(&order))
    );

    match Order::new(43, vec![]) {
        Err(ShopError::EmptyOrder) => println!("empty order rejected"),
        other => println!("unexpected: {other:?}"),
    }
}
