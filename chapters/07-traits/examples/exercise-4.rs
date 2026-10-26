//! Exercise 4: `Display` for `Product`: "Laptop — 45000.00 UAH"

use std::fmt;

struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // {:.2} — two decimal places, as in chapter 1
        write!(f, "{} — {:.2} UAH", self.name, self.price)
    }
}

fn main() {
    let product = Product {
        name: "Laptop".into(),
        price: 45_000.0,
    };
    println!("{product}"); // Laptop — 45000.00 UAH
}
