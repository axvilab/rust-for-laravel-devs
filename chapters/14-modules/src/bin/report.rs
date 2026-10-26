//! Окрема команда, як `php artisan shop:report`: кожен файл у `src/bin/` — свій бінарник.
//! `cargo run -p ch14-modules --bin report`

use ch14_modules::{models::Product, pricing};

fn main() {
    let items = [
        Product::new(1, "Ноутбук", 4_500_000),
        Product::new(2, "Миша", 80_000),
    ];
    for p in &items {
        println!("{:<10} {}", p.name, pricing::format_uah(p.price));
    }
}
