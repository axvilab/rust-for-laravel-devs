//! Вправа 4: `Display` для `Product`: "Ноутбук — 45000.00 грн"

use std::fmt;

struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // {:.2} — два знаки після коми, як у розділі 1
        write!(f, "{} — {:.2} грн", self.name, self.price)
    }
}

fn main() {
    let product = Product {
        name: "Ноутбук".into(),
        price: 45_000.0,
    };
    println!("{product}"); // Ноутбук — 45000.00 грн
}
