//! Exercise 1: a `Priced` trait with a default `price_with_vat`

trait Priced {
    fn price(&self) -> f64;
    fn price_with_vat(&self) -> f64 {
        self.price() * 1.2 // default implementation: 20% VAT
    }
}

struct Product {
    price: f64,
}

impl Priced for Product {
    fn price(&self) -> f64 {
        self.price
    }
}

fn main() {
    let p = Product { price: 100.0 };
    println!("{} → {}", p.price(), p.price_with_vat()); // 100 → 120
}
