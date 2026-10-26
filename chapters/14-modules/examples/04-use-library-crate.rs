//! The example uses this package's own library from outside: a hyphen in the name → an underscore
//!
//! The package `ch14-modules` is `ch14_modules` in code, like `tower-http` → `tower_http`.

use ch14_modules::models::{Order, Product};
use ch14_modules::services::PaymentService; // works thanks to `pub use` in services.rs
use ch14_modules::{calc_fee, pricing};

fn main() {
    let order = Order::new(1, vec![(Product::new(1, "Mouse", 80_000), 2)]).unwrap();
    let sub = pricing::subtotal(&order);
    println!(
        "{} + fee {}",
        pricing::format_uah(sub),
        pricing::format_uah(calc_fee(sub))
    );
    PaymentService::new()
        .charge("v@example.com", pricing::total_with_fee(&order))
        .unwrap();
    // pricing::round_to_hryvnia(1); // ERROR E0603: pub(crate) isn't visible outside the crate
}
