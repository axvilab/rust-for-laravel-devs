//! A separate command, like `php artisan shop:report`: every file in `src/bin/` is its own binary.
//! `cargo run -p ch14-modules --bin report`

use ch14_modules::{models::Product, pricing};

fn main() {
    let items = [
        Product::new(1, "Laptop", 4_500_000),
        Product::new(2, "Mouse", 80_000),
    ];
    for p in &items {
        println!("{:<10} {}", p.name, pricing::format_uah(p.price));
    }
}
