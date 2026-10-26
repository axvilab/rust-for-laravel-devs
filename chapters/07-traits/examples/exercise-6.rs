//! Exercise 6: `to_string()` without any new code
//!
//! Where the method comes from: the standard library has a blanket implementation
//! `impl<T: Display + ?Sized> ToString for T`. By implementing `Display`,
//! `Product` automatically got the `ToString` trait with its `to_string()` method.

use std::fmt;

struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} — {:.2} UAH", self.name, self.price)
    }
}

fn main() {
    let product = Product {
        name: "Laptop".into(),
        price: 45_000.0,
    };
    let s: String = product.to_string();
    println!("{s} ({} characters)", s.chars().count());
}
