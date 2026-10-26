//! Вправа 1: трейт `Priced` з дефолтним `price_with_vat`

trait Priced {
    fn price(&self) -> f64;
    fn price_with_vat(&self) -> f64 {
        self.price() * 1.2 // дефолтна реалізація: ПДВ 20%
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
