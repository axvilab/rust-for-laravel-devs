//! Вправа 6: `to_string()` без жодного нового коду
//!
//! Звідки метод: у стандартній бібліотеці є blanket-реалізація
//! `impl<T: Display + ?Sized> ToString for T`. Реалізувавши `Display`,
//! `Product` автоматично отримав трейт `ToString` з методом `to_string()`.

use std::fmt;

struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} — {:.2} грн", self.name, self.price)
    }
}

fn main() {
    let product = Product {
        name: "Ноутбук".into(),
        price: 45_000.0,
    };
    let s: String = product.to_string();
    println!("{s} ({} символів)", s.chars().count());
}
